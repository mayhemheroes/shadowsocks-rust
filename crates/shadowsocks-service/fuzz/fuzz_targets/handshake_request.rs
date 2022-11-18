#![no_main]
use libfuzzer_sys::fuzz_target;
use shadowsocks_service::local::socks::socks4::HandshakeRequest;

fuzz_target!(|data: &[u8]| {
    futures::executor::block_on(async {
        _ = HandshakeRequest::read_from(&mut data.clone()).await;
    });
});
