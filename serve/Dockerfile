# Use an official Python runtime as base image
FROM python:3.8-slim

# Set environment variables
ENV FLASK_APP=flask_server.py
ENV FLASK_RUN_HOST=0.0.0.0
ENV FLASK_RUN_PORT=80

# Install dependencies
COPY requirements.txt /application/serve/requirements.txt
WORKDIR /application/serve
RUN pip install --no-cache-dir -r requirements.txt

# Copy the current directory contents into the container at /app
COPY . /application/serve

# Run app.py when the container launches
CMD ["flask", "run", "--reload"]
