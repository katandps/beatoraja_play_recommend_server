FROM ubuntu
WORKDIR /app

COPY ./target/release/beatoraja_play_recommend_server /app
EXPOSE 80

CMD ["./beatoraja_play_recommend_server"]