LSM Aprende

Una app web para aprender Lengua de Señas Mexicana con la cámara de tu compu. La idea es : ves la seña, la imitas frente a la cámara, y la app te dice si la hiciste bien.
Apesar de que  , por el tiempo aun tiene algunos detalles , la idea esta bien estructurada y se hizo lo que se pudo jajaj

Por ahorita la app tiene tres módulos:

- **Abecedario** — las primeras letras del alfabeto en LSM
- **Saludos** — hola, adiós, gracias, por favor
- **Coloquial** — expresiones mexicanas de uso cotidiano

Cada lección tiene un video de referencia y la cámara activa para que practiques en tiempo real.Aunque por el momento no las 
reconozca del todo ya que , al existir señas parecidad, se confunde y las interpreta de forma incorecta 

¿Cómo funciona por dentro?

Usa **MediaPipe Hands** (una librería de Google) que detecta 21 puntos de tu mano a través de la cámara. Con esos puntos, la app analiza qué dedos están extendidos o doblados y decide qué seña estás haciendo. Si coincide con la lección que estás practicando, te da puntos 

¿Cómo correrlo?

```bash
git clone https://github.com/TU_USUARIO/TU_REPO.git
cd Proyecto_LSD/app
npm install
npm run dev


Luego abre `http://localhost:5173` en tu navegador y listo.


Tecnologías que usé

React + Vite
  MediaPipe Hands (detección de manos en el navegador)
  CSS puro para los estilos

---

Hecho por **Azul** como proyecto de accesibilidad para la comunidad sorda de México 🇲🇽
