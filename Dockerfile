FROM ubuntu
WORKDIR /app

COPY ./target/release/beatoraja_play_recommend_server /app
EXPOSE 80

ENTRYPOINT /app/beatoraja_play_recommend_server