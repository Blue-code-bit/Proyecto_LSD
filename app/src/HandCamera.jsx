// Importamos las herramientas de React que necesitamos
import { useEffect, useRef, useState } from 'react'

// Este componente maneja la cámara y detección de manos
// Usamos MediaPipe directamente desde su CDN en lugar de importarlo
// porque es más compatible con el navegador
function HandCamera({ onSenaDetectada, senaObjetivo }) {

  const videoRef = useRef(null)
  const canvasRef = useRef(null)
  const [camaraActiva, setCamaraActiva] = useState(false)
  const [estado, setEstado] = useState('Presiona "Activar cámara" para comenzar')


  // FUNCIÓN: Analizar la posición de los dedos
  // Recibe los 21 puntos de la mano y decide qué seña es
  
  const detectarSena = (landmarks) => {
    const indice = landmarks[8]
    const medio = landmarks[12]
    const anular = landmarks[16]
    const menique = landmarks[20]
    const indiceMedio = landmarks[6]
    const medioMedio = landmarks[10]
    const anularMedio = landmarks[14]
    const meniqueMedio = landmarks[18]

    // Un dedo está extendido si su punta está más arriba que su articulación
    const indiceExtendido = indice.y < indiceMedio.y
    const medioExtendido = medio.y < medioMedio.y
    const anularExtendido = anular.y < anularMedio.y
    const meniqueExtendido = menique.y < meniqueMedio.y

    // LETRA A: Puño cerrado, ningún dedo extendido
    if (!indiceExtendido && !medioExtendido && !anularExtendido && !meniqueExtendido) {
      return 'Letra A'
    }
    // LETRA B: Los 4 dedos extendidos
    if (indiceExtendido && medioExtendido && anularExtendido && meniqueExtendido) {
      return 'Letra B'
    }
    // LETRA C: Índice y medio extendidos
    if (indiceExtendido && medioExtendido && !anularExtendido && !meniqueExtendido) {
      return 'Letra C'
    }
    // LETRA D: Solo índice extendido
    if (indiceExtendido && !medioExtendido && !anularExtendido && !meniqueExtendido) {
      return 'Letra D'
    }
    // LETRA E: Solo meñique extendido
    if (!indiceExtendido && !medioExtendido && !anularExtendido && meniqueExtendido) {
      return 'Letra E'
    }

    return null
  }

  
  // FUNCIÓN: Cargar MediaPipe desde CDN y activar la cámara

  const activarCamara = () => {
    setEstado('Cargando MediaPipe...')

    // Creamos un script que carga MediaPipe desde internet
    // Es como agregar <script src="..."> en el HTML pero desde React
    const script = document.createElement('script')
    script.src = 'https://cdn.jsdelivr.net/npm/@mediapipe/hands/hands.js'
    script.crossOrigin = 'anonymous'

    // Cuando el script termine de cargar, iniciamos la cámara
    script.onload = () => {
      const script2 = document.createElement('script')
      script2.src = 'https://cdn.jsdelivr.net/npm/@mediapipe/camera_utils/camera_utils.js'
      script2.crossOrigin = 'anonymous'

      script2.onload = () => {
        const script3 = document.createElement('script')
        script3.src = 'https://cdn.jsdelivr.net/npm/@mediapipe/drawing_utils/drawing_utils.js'
        script3.crossOrigin = 'anonymous'

        script3.onload = () => {
          // Ya cargaron todos los scripts, iniciamos la detección
          iniciarDeteccion()
        }
        document.body.appendChild(script3)
      }
      document.body.appendChild(script2)
    }
    document.body.appendChild(script)
  }

  
  // FUNCIÓN: Iniciar la detección de manos
  // Se llama después de que MediaPipe cargó correctament
  const iniciarDeteccion = () => {
    // Accedemos a MediaPipe que ya está en el objeto global "window"
    const Hands = window.Hands
    const Camera = window.Camera
    const drawConnectors = window.drawConnectors
    const drawLandmarks = window.drawLandmarks
    const HAND_CONNECTIONS = window.HAND_CONNECTIONS

    // Configuramos MediaPipe Hands
    const hands = new Hands({
      locateFile: (file) => {
        return `https://cdn.jsdelivr.net/npm/@mediapipe/hands/${file}`
      }
    })

    hands.setOptions({
      maxNumHands: 1,
      modelComplexity: 1,
      minDetectionConfidence: 0.7,
      minTrackingConfidence: 0.5
    })

    // Esta función se ejecuta en cada frame del video
    hands.onResults((results) => {
      const canvas = canvasRef.current
      if (!canvas) return
      const ctx = canvas.getContext('2d')

      ctx.clearRect(0, 0, canvas.width, canvas.height)
      ctx.drawImage(results.image, 0, 0, canvas.width, canvas.height)

      if (results.multiHandLandmarks && results.multiHandLandmarks.length > 0) {
        const landmarks = results.multiHandLandmarks[0]

        // Dibujamos las conexiones entre puntos de la mano
        drawConnectors(ctx, landmarks, HAND_CONNECTIONS, {
          color: '#9945ff',
          lineWidth: 3
        })

        // Dibujamos los 21 puntos de la mano
        drawLandmarks(ctx, landmarks, {
          color: '#14f195',
          lineWidth: 1,
          radius: 5
        })

        // Analizamos qué seña está haciendo el usuario
        const senaDetectada = detectarSena(landmarks)

        if (senaDetectada) {
          setEstado(`Detectando: ${senaDetectada}`)
          if (senaDetectada === senaObjetivo) {
            setEstado(`✅ ¡Correcto! Hiciste: ${senaDetectada}`)
            onSenaDetectada(senaDetectada)
          }
        } else {
          setEstado('Mano detectada — sigue intentando...')
        }
      } else {
        setEstado('No detecto tu mano — acércala a la cámara')
      }
    })

    // Iniciamos la cámara
    const camera = new Camera(videoRef.current, {
      onFrame: async () => {
        await hands.send({ image: videoRef.current })
      },
      width: 480,
      height: 360
    })

    camera.start()
    setCamaraActiva(true)
    setEstado('Cámara activa — muestra tu mano')
  }

  return (
    <div className="hand-camera">

      {/* Botón para activar la cámara */}
      {!camaraActiva && (
        <button className="btn-activar-camara" onClick={activarCamara}>
          📷 Activar cámara
        </button>
      )}

      <div className="camara-wrapper">
        {/* Video oculto que MediaPipe usa internamente */}
        <video
          ref={videoRef}
          style={{ display: 'none' }}
          playsInline
        />

        {/* Canvas donde se muestra el video con los puntos encima */}
        <canvas
          ref={canvasRef}
          width={480}
          height={360}
          className="camara-canvas"
        />
      </div>

      {/* Mensaje de estado para guiar al usuario */}
      <p className="estado-camara">{estado}</p>

    </div>
  )
}

export default HandCamera