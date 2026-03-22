# Proyecto_LSM
Objetivo de crear un programa interactivo para enseñar Lenguaje De Señas mediante una camara web y
tecnología blockchain de Solana.

LSM Aprende combina visión por computadora con blockchain para crear una experiencia única de aprendizaje:
- **Detección de manos en tiempo real** usando MediaPipe de Google
  **Progreso guardado en blockchain** usando Solana + Anchor
- **Certificados on-chain** al completar las lecciones
- **Lecciones interactivas** del abecedario LSM

## Tecnologías usadas

### Blockchain (on-chain)
- **Rust + Anchor** — Smart contract en Solana
- **Solana Devnet** — Red de pruebas

### Frontend (off-chain)
- **React + Vite** — Interfaz de usuario
- **MediaPipe Hands** — Detección de manos con IA
- **Solana Wallet Adapter** — Conexión con wallets

## Funcionalidades del Smart Contract

| Instrucción | Descripción |
|---|---|
| `crear_leccion` | Crea una lección de señas on-chain |
| `actualizar_leccion` | Actualiza el contenido de una lección |
| `borrar_leccion` | Elimina una lección de blockchain |
| `registrar_usuario` | Crea el perfil del estudiante |
| `completar_leccion` | Suma 10 puntos al completar una seña |
| `reclamar_certificado` | Emite un certificado al llegar a 100 pts |

## Cómo correr el proyecto localmente

### Requisitos
- Node.js v18+
- Rust
- Solana CLI
- Anchor CLI

### Pasos
```bash
# Clonar el repositorio
git clone https://github.com/Blue-code-bit/Proyecto_LSD.git
cd Proyecto_LSD

# Instalar dependencias del frontend
cd app
npm install

# Correr el frontend
npm run dev
```

Abre `http://localhost:5173` en tu navegador.

## Program ID en Solana Devnet
```
9jgxxoABZciCdDN4f9Stwv9sTaJkQmyT682NtR4bryME
```

## Autor

**Azul** — [@Blue-code-bit](https://github.com/Blue-code-bit)

Proyecto desarrollado para WayLearn Latam — comunidad Web3 en Latinoamérica.
