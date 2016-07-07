use self::mio::ffmpeg_mio;

fn main() {
    let ffmpeg_server = FfmpegServer::new();
    let title: str = "Test Title".as_str();
    let ffmpeg_stream = FfmpegStream::new(title);
    let server_registered = ffmpeg_server.register_ffmpeg_server();
    server_registered.ready();
}
