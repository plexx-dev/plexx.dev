FROM nginx:alpine

COPY * /usr/share/nginx/html/
COPY css/ /usr/share/nginx/html/css/
COPY webfonts/ /usr/share/nginx/html/css/webfonts/