FROM nginx:alpine

COPY . /usr/share/nginx/html/

RUN ls /usr/share/nginx/html
RUN ls /usr/share/nginx/html/css