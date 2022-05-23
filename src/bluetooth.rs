use bluer::{
    adv::Advertisement,
    gatt::{
        local::{
            characteristic_control, Application, Characteristic as LocalCharacteristic,
            CharacteristicControlEvent, CharacteristicNotify, CharacteristicNotifyMethod, CharacteristicWrite,
            CharacteristicWriteMethod, Service,
        },
        remote::Characteristic as RemoteCharacteristic,
        CharacteristicReader, CharacteristicWriter,
    },
    AdapterEvent, Device,
};
use futures::{future, pin_mut, StreamExt};
use std::time::Duration;
use tokio::{
    io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader},
    time::{sleep, timeout},
};

pub async fn start_bt_server() -> bluer::Result<()> {
    let session = bluer::Session::new().await?;
    let adapter = session.default_adapter().await?;
    adapter.set_powered(true).await?;

    println!(
        "Advertising on Bluetooth adapter {} with address {}",
        adapter.name(),
        adapter.address().await?
    );
    let le_advertisement = Advertisement {
        service_uuids: vec![crate::consts::APP_BT_SERVICE_ID].into_iter().collect(),
        discoverable: Some(true),
        local_name: Some("gatt_echo_server".to_string()),
        ..Default::default()
    };
    let adv_handle = adapter.advertise(le_advertisement).await?;

    println!(
        "Serving GATT echo service on Bluetooth adapter {}",
        adapter.name()
    );
    let (char_control, char_handle) = characteristic_control();
    let app = Application {
        services: vec![Service {
            uuid: crate::consts::APP_BT_SERVICE_ID,
            primary: true,
            characteristics: vec![LocalCharacteristic {
                uuid: crate::consts::APP_BT_CHARACTERISTIC_ID,
                write: Some(CharacteristicWrite {
                    write_without_response: true,
                    method: CharacteristicWriteMethod::Io,
                    ..Default::default()
                }),
                notify: Some(CharacteristicNotify {
                    notify: true,
                    method: CharacteristicNotifyMethod::Io,
                    ..Default::default()
                }),
                control_handle: char_handle,
                ..Default::default()
            }],
            ..Default::default()
        }],
        ..Default::default()
    };
    let app_handle = adapter.serve_gatt_application(app).await?;

    println!("Echo service ready. Press enter to quit.");
    let stdin = BufReader::new(tokio::io::stdin());
    let mut lines = stdin.lines();

    let mut read_buf = Vec::new();
    let mut reader_opt: Option<CharacteristicReader> = None;
    let mut writer_opt: Option<CharacteristicWriter> = None;
    pin_mut!(char_control);

    loop {
        tokio::select! {
            _ = lines.next_line() => break,
            evt = char_control.next() => {
                match evt {
                    Some(CharacteristicControlEvent::Write(req)) => {
                        println!("Accepting write request event with MTU {}", req.mtu());
                        read_buf = vec![0; req.mtu()];
                        reader_opt = Some(req.accept()?);
                    },
                    Some(CharacteristicControlEvent::Notify(notifier)) => {
                        println!("Accepting notify request event with MTU {}", notifier.mtu());
                        writer_opt = Some(notifier);
                    },
                    None => break,
                }
            },
            read_res = async {
                match &mut reader_opt {
                    Some(reader) if writer_opt.is_some() => reader.read(&mut read_buf).await,
                    _ => future::pending().await,
                }
            } => {
                match read_res {
                    Ok(0) => {
                        println!("Read stream ended");
                        reader_opt = None;
                    }
                    Ok(n) => {
                        let value = read_buf[..n].to_vec();
                        println!("Echoing {} bytes: {:x?} ... {:x?}", value.len(), &value[0..4.min(value.len())], &value[value.len().saturating_sub(4) ..]);
                        if value.len() < 512 {
                            println!();
                        }
                        if let Err(err) = writer_opt.as_mut().unwrap().write_all(&value).await {
                            println!("Write failed: {}", &err);
                            writer_opt = None;
                        }
                    }
                    Err(err) => {
                        println!("Read stream error: {}", &err);
                        reader_opt = None;
                    }
                }
            }
        }
    }

    println!("Removing service and advertisement");
    drop(app_handle);
    drop(adv_handle);
    sleep(Duration::from_secs(1)).await;

    Ok(())
}

async fn find_our_characteristic(device: &Device) -> bluer::Result<Option<RemoteCharacteristic>> {
    let addr = device.address();
    let uuids = device.uuids().await?.unwrap_or_default();
    println!("Discovered device {} with service UUIDs {:?}", addr, &uuids);
    let md = device.manufacturer_data().await?;
    println!("    Manufacturer data: {:x?}", &md);

    if uuids.contains(&crate::consts::APP_BT_SERVICE_ID) {
        println!("    Device provides our service!");
        if !device.is_connected().await? {
            println!("    Connecting...");
            let mut retries = 2;
            loop {
                match device.connect().await {
                    Ok(()) => break,
                    Err(err) if retries > 0 => {
                        println!("    Connect error: {}", &err);
                        retries -= 1;
                    }
                    Err(err) => return Err(err),
                }
            }
            println!("    Connected");
        } else {
            println!("    Already connected");
        }

        println!("    Enumerating services...");
        for service in device.services().await? {
            let uuid = service.uuid().await?;
            println!("    Service UUID: {}", &uuid);
            if uuid == crate::consts::APP_BT_SERVICE_ID {
                println!("    Found our service!");
                for char in service.characteristics().await? {
                    let uuid = char.uuid().await?;
                    println!("    Characteristic UUID: {}", &uuid);
                    if uuid == crate::consts::APP_BT_CHARACTERISTIC_ID {
                        println!("    Found our characteristic!");
                        return Ok(Some(char));
                    }
                }
            }
        }

        println!("    Not found!");
    }

    Ok(None)
}

async fn send_server_data(char: &bluer::gatt::remote::Characteristic, data: Vec<u8>) -> bluer::Result<()> {
    let mut write_io = char.write_io().await?;
    println!("    Obtained write IO with MTU {} bytes", write_io.mtu());
    let mut notify_io = char.notify_io().await?;
    println!("    Obtained notification IO with MTU {} bytes", notify_io.mtu());

    // Flush notify buffer.
    let mut buf = [0; 1024];
    while let Ok(Ok(_)) = timeout(Duration::from_secs(1), notify_io.read(&mut buf)).await {}

    let data_len = data.len();
    // We must read back the data while sending, otherwise the connection
    // buffer will overrun and we will lose data.
    let read_task = tokio::spawn(async move {
        let mut echo_buf = vec![0u8; data_len];
        let res = match notify_io.read_exact(&mut echo_buf).await {
            Ok(_) => Ok(echo_buf),
            Err(err) => Err(err),
        };
        (notify_io, res)
    });

    // Note that write_all will automatically split the buffer into
    // multiple writes of MTU size.
    write_io.write_all(&data).await.expect("write failed");

    println!("    Waiting for echo");
    let (notify_io_back, res) = read_task.await.unwrap();
    notify_io = notify_io_back;
    let echo_buf = res.expect("read failed");

    if echo_buf != data {
        println!();
        println!("Echo data mismatch!");
        println!("Send data:     {:x?}", &data);
        println!("Received data: {:x?}", &echo_buf);
        println!();
        println!("By 512 blocks:");
        for (sent, recv) in data.chunks(512).zip(echo_buf.chunks(512)) {
            println!();
            println!(
                "Send: {:x?} ... {:x?}",
                &sent[0..4.min(sent.len())],
                &sent[sent.len().saturating_sub(4)..]
            );
            println!(
                "Recv: {:x?} ... {:x?}",
                &recv[0..4.min(recv.len())],
                &recv[recv.len().saturating_sub(4)..]
            );
        }
        println!();

        panic!("echoed data does not match sent data");
    }
    println!("    Data was sent to server");
    Ok(())
}

pub async fn send_request_to_server(data: Vec<u8>) -> bluer::Result<()> {
    let session = bluer::Session::new().await?;
    let adapter = session.default_adapter().await?;
    adapter.set_powered(true).await?;

    {
        println!(
            "Discovering on Bluetooth adapter {} with address {}\n",
            adapter.name(),
            adapter.address().await?
        );
        let discover = adapter.discover_devices().await?;
        pin_mut!(discover);
        let mut done = false;
        while let Some(evt) = discover.next().await {
            match evt {
                AdapterEvent::DeviceAdded(addr) => {
                    let device = adapter.device(addr)?;
                    match find_our_characteristic(&device).await {
                        Ok(Some(char)) => {
                            // FIXME should we really have to clone here?
                            match send_server_data(&char, data.clone()).await {
                                Ok(()) => {
                                    println!("    Characteristic exercise completed");
                                    done = true;
                                }
                                Err(err) => {
                                    println!("    Characteristic exercise failed: {}", &err);
                                }
                            }
                        }
                        Ok(None) => (),
                        Err(err) => {
                            println!("    Device failed: {}", &err);
                            let _ = adapter.remove_device(device.address()).await;
                        }
                    }
                    match device.disconnect().await {
                        Ok(()) => println!("    Device disconnected"),
                        Err(err) => println!("    Device disconnection failed: {}", &err),
                    }
                    println!();
                }
                AdapterEvent::DeviceRemoved(addr) => {
                    println!("Device removed {}", addr);
                }
                _ => (),
            }
            if done {
                break;
            }
        }
        println!("Stopping discovery");
    }

    sleep(Duration::from_secs(1)).await;
    Ok(())
}
