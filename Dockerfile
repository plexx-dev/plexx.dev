FROM nginx:alpine

COPY . /usr/share/nginx/html/
RUN cp -f /usr/share/nginx/html/nginx.conf /etc/nginx/nginx.conf
