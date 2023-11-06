# Use HAProxy's official image
FROM haproxy:2.4

# Copy the configuration file into the image
COPY haproxy.cfg /usr/local/etc/haproxy/haproxy.cfg

# Copy your SSL cert and key
COPY that.photo.pem /etc/ssl/private/that.photo.pem

