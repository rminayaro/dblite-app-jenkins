# Etapa 1: Construcción
FROM rust:latest AS builder

WORKDIR /usr/src/app
COPY . .

# Instalar dependencias para compilar correctamente con OpenSSL
RUN apt-get update && apt-get install -y pkg-config libssl-dev && cargo build --release

# Etapa 2: Imagen final con dependencias mínimas
FROM debian:latest
WORKDIR /usr/src/app

# Instalar OpenSSL en la imagen final
RUN apt-get update && apt-get install -y libssl3 ca-certificates

# Crear la carpeta sqlite dentro del contenedor
RUN mkdir -p /usr/src/app/sqlite

# Copiar el binario de la aplicación
COPY --from=builder /usr/src/app/target/release/dblite_app .

# Copiar la base de datos al contenedor
COPY sqlite/InventarioRopa.db /usr/src/app/sqlite/InventarioRopa.db

# Asegurar permisos de lectura y escritura en la base de datos
RUN chmod 666 /usr/src/app/sqlite/InventarioRopa.db

# Exponer el puerto y ejecutar
EXPOSE 3030
CMD ["./dblite_app"]
