# 🎵 ShinsunMusic Backend

![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)
![MongoDB](https://img.shields.io/badge/MongoDB-%234ea94b.svg?style=for-the-badge&logo=mongodb&logoColor=white)
![Axum](https://img.shields.io/badge/Axum-000000?style=for-the-badge&logo=rust&logoColor=white)

A robust backend service for ShinsunMusic application, built with Rust using Domain-Driven Design principles and clean architecture patterns.

## 🏗️ Architecture Overview

This project follows Clean Architecture and Domain-Driven Design principles:

```
src/
├── domain/          # Business logic and domain models
├── application/     # Application services and use cases
├── infrastructure/  # External services implementation
└── interfaces/      # API endpoints and controllers
```

## 🚀 Key Features

- **Domain-Driven Design (DDD)** architecture
- **Clean Architecture** principles
- Built with **Axum** web framework
- **MongoDB** integration with strong typing
- **JWT** authentication and authorization
- Comprehensive **middleware** stack
- **CORS** configuration
- **Dependency Injection** pattern
- **Shared state** management
- **OpenAPI** documentation
- **Tracing** and logging system

## 🛠️ Tech Stack

- **Framework:** Axum
- **Database:** MongoDB
- **Authentication:** JWT
- **Documentation:** OpenAPI/Swagger
- **Logging:** tracing, tracing-subscriber
- **Configuration:** config-rs
- **Validation:** validator
- **Error Handling:** thiserror
- **Serialization:** serde
- **Testing:** tokio-test

## 📦 Prerequisites

- Rust (latest stable version)
- MongoDB
- Docker (optional)

## 🚦 Getting Started

1. **Clone the repository**
```bash
git clone https://github.com/yourusername/shinsunmusic-backend.git
cd shinsunmusic-backend
```

2. **Set up environment variables**
```bash
cp .env.example .env
# Edit .env with your configuration
```

3. **Run the development server**
```bash
cargo run
```

4. **Run with Docker**
```bash
docker-compose up -d
```

## 🧪 Testing

```bash
# Run unit tests
cargo test

# Run integration tests
cargo test --test '*'
```

## 📚 API Documentation

API documentation is available at `/swagger-ui` when the server is running.

### Key Endpoints

- `POST /api/auth/login` - User authentication
- `GET /api/songs` - List all songs
- `POST /api/playlists` - Create new playlist
- `GET /api/users/me` - Get current user profile

## 🔒 Security

- CORS configuration
- Rate limiting
- JWT authentication
- Request validation
- Input sanitization

## 🔧 Configuration

Configuration is managed through environment variables and `config.rs`:

```toml
[server]
host = "127.0.0.1"
port = 8080

[database]
url = "mongodb://localhost:27017"
name = "shinsunmusic"
```

## 📊 Monitoring and Logging

- Structured logging with tracing
- Request/Response logging middleware
- Error tracking
- Performance metrics

## 🤝 Contributing

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/AmazingFeature`)
3. Commit your changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

## 📝 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.