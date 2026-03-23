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
  // Cada punto tiene coordenadas x, y entre 0 y 1
  
  const detectarSena = (landmarks) => {

    // Sacamos los puntos clave de la mano
    const muneca = landmarks[0]  // Base de la mano
    const pulgarPunta = landmarks[4] // Punta del pulgar
    const indicePunta = landmarks[8] // Punta del índice
    const medioPunta = landmarks[12] // Punta del dedo medio
    const anularPunta = landmarks[16]// Punta del anular
    const meniquePunta = landmarks[20] // Punta del meñique

    // Articulaciones medias de cada dedo
    const pulgarMedio = landmarks[3]
    const indiceMedio = landmarks[6]
    const medioMedio = landmarks[10]
    const anularMedio = landmarks[14]
    const meniqueMedio = landmarks[18]

    // Base de cada dedo (donde se une con la palma)
    const indiceBase = landmarks[5]
    const medioBase = landmarks[9]

    
    // Calculamos si cada dedo está extendido o doblado
    // Un dedo está extendido si su punta está más arriba
    // que su articulación media (y menor = más arriba)
    
    const indiceExtendido = indicePunta.y < indiceMedio.y
    const medioExtendido = medioPunta.y < medioMedio.y
    const anularExtendido = anularPunta.y < anularMedio.y
    const meniqueExtendido = meniquePunta.y < meniqueMedio.y

    // El pulgar se mide diferente porque se mueve en horizontal
    const pulgarExtendido = pulgarPunta.x < pulgarMedio.x

    // Calculamos la distancia entre dos puntos
    // Útil para detectar si dos dedos están juntos o separados
    const distancia = (p1, p2) => {
      return Math.sqrt(
        Math.pow(p1.x - p2.x, 2) +
        Math.pow(p1.y - p2.y, 2)
      )
    }

    
    // ABECEDARIO LSM
    

    // LETRA A: Puño cerrado, ningún dedo extendido
    if (!indiceExtendido && !medioExtendido && !anularExtendido && !meniqueExtendido) {
      return 'Letra A'
    }

    // LETRA B: Los 4 dedos extendidos y juntos
    if (indiceExtendido && medioExtendido && anularExtendido && meniqueExtendido) {
      const dedosJuntos = distancia(indicePunta, medioPunta) < 0.05
      if (dedosJuntos) return 'Letra B'
    }

    // LETRA C: Solo índice y medio extendidos y separados
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

    
    // SALUDOS


    // HOLA: Mano abierta con todos los dedos extendidos y separados
    if (indiceExtendido && medioExtendido && anularExtendido && meniqueExtendido) {
      const dedosSeparados = distancia(indicePunta, medioPunta) > 0.05
      if (dedosSeparados) return 'Hola'
    }

    // ADIÓS: Igual que hola pero la mano inclinada
    // Lo detectamos cuando la muñeca está más alta que la base del índice
    if (indiceExtendido && medioExtendido && anularExtendido && meniqueExtendido) {
      const manoInclinada = muneca.y < indiceBase.y
      if (manoInclinada) return 'Adiós'
    }

    // POR FAVOR: Mano abierta con palma hacia arriba
    // El medio está más alto que la muñeca
    if (indiceExtendido && medioExtendido && anularExtendido && meniqueExtendido) {
      const palmaArriba = medioPunta.y < muneca.y - 0.1
      if (palmaArriba) return 'Por favor'
    }

    // GRACIAS: Índice y medio extendidos tocando cerca de la cara
    // Los detectamos cuando la punta del índice está muy arriba
    if (indiceExtendido && medioExtendido && !anularExtendido && !meniqueExtendido) {
      const manoArriba = indicePunta.y < 0.3
      if (manoArriba) return 'Gracias'
    }

    
    // BUENOS DÍAS / TARDES / NOCHES
    

    // BUENOS DÍAS: Ambas manos abiertas hacia arriba
    // Lo simulamos con mano abierta apuntando hacia arriba
    if (indiceExtendido && medioExtendido && anularExtendido && meniqueExtendido) {
      const apuntaArriba = indicePunta.y < 0.2
      if (apuntaArriba) return 'Buenos días'
    }

    // BUENAS TARDES: Mano horizontal moviéndose hacia abajo
    // Lo detectamos con mano abierta a altura media
    if (indiceExtendido && medioExtendido && anularExtendido && meniqueExtendido) {
      const alturaMedia = indicePunta.y > 0.3 && indicePunta.y < 0.6
      if (alturaMedia) return 'Buenas tardes'
    }

    // BUENAS NOCHES: Manos cruzadas frente al pecho
    // Lo detectamos con puño cerrado a altura del pecho
    if (!indiceExtendido && !medioExtendido && !anularExtendido && !meniqueExtendido) {
      const alturaPecho = muneca.y > 0.4 && muneca.y < 0.7
      if (alturaPecho) return 'Buenas noches'
    }

    // HASTA MAÑANA: Índice extendido apuntando hacia adelante
    if (indiceExtendido && !medioExtendido && !anularExtendido && !meniqueExtendido) {
      const apuntaAdelante = indicePunta.y > 0.4
      if (apuntaAdelante) return 'Hasta mañana'
    }

    
    // GROCERIAS


    // WEY: Índice y meñique extendidos (cuernos)
    if (indiceExtendido && !medioExtendido && !anularExtendido && meniqueExtendido) {
      return 'Wey'
    }

    // NACO: Pulgar hacia abajo
    if (pulgarExtendido && !indiceExtendido && !medioExtendido && !anularExtendido && !meniqueExtendido) {
      const pulgarAbajo = pulgarPunta.y > muneca.y
      if (pulgarAbajo) return 'Naco'
    }

    // METICHE: Índice apuntando hacia adelante a altura media
    if (indiceExtendido && !medioExtendido && !anularExtendido && !meniqueExtendido) {
      const alturaMedia = indicePunta.y > 0.35 && indicePunta.y < 0.65
      if (alturaMedia) return 'Metiche'
    }

    // CHISMOSO: Mano en C cerca de la boca
    // Detectamos índice y pulgar formando una O/C
    if (!medioExtendido && !anularExtendido && !meniqueExtendido) {
      const formandoC = distancia(indicePunta, pulgarPunta) < 0.08
      if (formandoC) return 'Chismoso'
    }

    // CHAFA: Pulgar hacia abajo moviéndose
    if (pulgarExtendido && !indiceExtendido && !medioExtendido) {
      return 'Chafa'
    }

    // Si no coincide con ninguna seña
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