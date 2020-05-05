FROM ubuntu
WORKDIR /app

COPY ./target/release/beatoraja_play_recommend_server /app
EXPOSE 8000

CMD ["./beatoraja_play_recommend_server"]