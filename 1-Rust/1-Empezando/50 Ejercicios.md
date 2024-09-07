# Rust Programming Exercises

## 1. Workspace with Multiple Libraries

### Objective
Create a Rust workspace containing one binary application and two libraries. The main program should utilize both libraries.

### Steps
1. Create a new workspace:
   ```bash
   mkdir rust_workspace && cd rust_workspace
   ```

2. Initialize the workspace:
   ```bash
   cargo init --bin app
   cargo new --lib lib1
   cargo new --lib lib2
   ```

3. Edit `Cargo.toml` in the root directory:
   ```toml
   [workspace]
   members = [
       "app",
       "lib1",
       "lib2"
   ]
   ```

4. Implement functionality in `lib1` and `lib2`.

5. Update `app/Cargo.toml` to include dependencies:
   ```toml
   [dependencies]
   lib1 = { path = "../lib1" }
   lib2 = { path = "../lib2" }
   ```

6. Use both libraries in `app/src/main.rs`.

## 2. Web Server with Docker

### Objective
Create a web server with two GET routes, dockerize it, and publish it on port 8888.

### Steps
1. Create a new Rust project:
   ```bash
   cargo new web_server && cd web_server
   ```

2. Add dependencies to `Cargo.toml`:
   ```toml
   [dependencies]
   actix-web = "4.0"
   ```

3. Implement the web server in `src/main.rs`:
   ```rust
   use actix_web::{get, App, HttpServer, HttpResponse, Responder};

   #[get("/route1")]
   async fn route1() -> impl Responder {
       HttpResponse::Ok().body("Route 1")
   }

   #[get("/route2")]
   async fn route2() -> impl Responder {
       HttpResponse::Ok().body("Route 2")
   }

   #[actix_web::main]
   async fn main() -> std::io::Result<()> {
       HttpServer::new(|| {
           App::new()
               .service(route1)
               .service(route2)
       })
       .bind("0.0.0.0:8888")?
       .run()
       .await
   }
   ```

4. Create a `Dockerfile`:
   ```dockerfile
   FROM rust:1.68 as builder
   WORKDIR /usr/src/app
   COPY . .
   RUN cargo build --release

   FROM debian:buster-slim
   COPY --from=builder /usr/src/app/target/release/web_server /usr/local/bin/
   EXPOSE 8888
   CMD ["web_server"]
   ```

5. Build and run the Docker container:
   ```bash
   docker build -t rust-web-server .
   docker run -p 8888:8888 rust-web-server
   ```

## 3. Kubernetes Deployment (Optional)

### Objective
Deploy the Docker image to a Kubernetes cluster using an Ingress, Service, and Pod.

### Steps
1. Create a Kubernetes deployment file `deployment.yaml`:
   ```yaml
   apiVersion: apps/v1
   kind: Deployment
   metadata:
     name: rust-web-server
   spec:
     replicas: 1
     selector:
       matchLabels:
         app: rust-web-server
     template:
       metadata:
         labels:
           app: rust-web-server
       spec:
         containers:
         - name: rust-web-server
           image: rust-web-server:latest
           ports:
           - containerPort: 8888
   ---
   apiVersion: v1
   kind: Service
   metadata:
     name: rust-web-server-service
   spec:
     selector:
       app: rust-web-server
     ports:
     - protocol: TCP
       port: 80
       targetPort: 8888
   ---
   apiVersion: networking.k8s.io/v1
   kind: Ingress
   metadata:
     name: rust-web-server-ingress
   spec:
     rules:
     - http:
         paths:
         - path: /
           pathType: Prefix
           backend:
             service:
               name: rust-web-server-service
               port: 
                 number: 80
   ```

2. Apply the configuration:
   ```bash
   kubectl apply -f deployment.yaml
   ```

3. Verify the deployment:
   ```bash
   kubectl get pods,services,ingress
   ```

This improved version provides a clearer structure, more detailed steps, and code snippets for each exercise. It also includes the Kubernetes deployment as an optional step with a complete YAML configuration.