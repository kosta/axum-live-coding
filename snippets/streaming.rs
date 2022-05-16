async fn streaming(body: BodyStream) -> impl IntoResponse {
    let reader = StreamReader::new(body.map_err(|e| io::Error::new(io::ErrorKind::Other, e)));
    let lines = LinesStream::new(reader.lines());
    let reversed_lines = lines.map_ok(|line| {
        let mut line: String = line.chars().into_iter().rev().collect();
        line.push('\n');
        Bytes::from(line)
    });
    StreamBody::new(reversed_lines)
}