FROM nginx:alpine

COPY . /usr/share/nginx/html/
RUN cp -f /usr/share/html/nginx.conf /etc/nginx/nginx.conf
