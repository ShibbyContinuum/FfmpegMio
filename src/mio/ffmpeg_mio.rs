use mio::*;
use std::io::Write;

const FFMPEG_STREAM: Token = mio::Token(0);

struct FfmpegStream {
    mio: Node,
    stream: Stream
}

pub struct Stream {
    title: String,
    fd_0: Stdin,
    fd_1: Stdout,
    fd_2: Stderr
}

pub struct Node {
    addr: Ipv4Addr,
    server: TcpStream,
    port: u16,
    event_loop: EventLoop<FfmpegStream>
}

impl Node {
    pub fn new() -> Node {
        let addr = Ipv4Addr::new(127,0,0,1);
        let node = Node {
            addr: addr,
            server: TcpStream::connect(addr),
            port: 12000,
            event_loop: EventLoop<FfmpegStream>
        };
        node
    }
}

impl Stream {
    pub fn new() -> Stream {
        let title = "Test Title".to_string();
        let ffmpeg = Command::new("sh").arg("-c")
                                       .arg("ffmpeg")
                                       .arg("-i")
                                       .arg("./source/test/video")
                                       .arg("./output")
                                       .output()
                                       .unwrap_or_else(|e| { println!("{}", e) });

        let stream = Stream {
            title: title,
            fd_0: ffmpeg.stdin,
            fd_1: ffmpeg.stdout,
            fd_2: ffmpeg.stderr
        };
        ffmpeg_stream
    }
}
//  let ffmpeg_stream: FfmpegStream = FfmpegStream::new();
//  let ffmpeg_server: FfmpegStream = FfmpegServer::new();
//
impl Handler for FfmpegStream {
    type Timeout = ();
    type Message = ();
    pub fn new() -> FfmpegStream {
        let node = Node::new();
        let stream = Stream::new();
        let ffmpeg_stream = FfmpegStream {
            mio: node,
            stream: stream
        };
        ffmpeg_stream
    }

    fn register_ffmpeg_server(&self) {
        let register_event_loop = ffmpeg_server.register(&self.mio, FFMPEG_SERVER, EventSet::readable(), PollOpt::edge());
        match register_event_loop {
            Ok(success) => success,
            Err(e) => println!("This should go to a log file: {}", e),
        };
    }

    fn ready(&mut self, event_loop: &mut EventLoop<FfmpegStream>, token: Token, events: EventSet) {
        match token {
            FFMPEG_STREAM => {
                let FfmpegStream(ref mut server) = {
                    *self.stream.fd_1.write_all(&self.mio.server);
                };
                let _= server.accept();
            }
            _ => panic!("unexpected token"),
        }
    }
}
