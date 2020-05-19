FROM python:3 AS dev

COPY requirements.txt .
RUN pip install --no-cache-dir -r requirements.txt

CMD ["python3", "/var/www/app.py"]

FROM python:3

COPY requirements.txt .
RUN pip install --no-cache-dir -r requirements.txt

WORKDIR /var/www
COPY . .

CMD ["uwsgi","--ini","/var/www/uwsgi.ini"]
