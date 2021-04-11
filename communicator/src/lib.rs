#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

//이 library는 network, client 모듈을 가지고 있다
mod network;
mod client;
