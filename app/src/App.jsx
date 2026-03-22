// Importamos las herramientas de React que necesitamos
// useState = para guardar datos que cambian (como el nombre del usuario)
// useEffect = para ejecutar código cuando la página carga
import { useState, useEffect, useRef } from 'react'

// Importamos los estilos de la página
import './App.css'

// Importamos el componente de la cámara que acabamos de crear
import HandCamera from './HandCamera'

// Este es el componente principal de la aplicación
// Todo lo que escribamos aquí aparecerá en la página web
function App() {


  // VARIABLES DE ESTADO
  // Son como "cajitas" que guardan información.
  // Cuando cambian, la página se actualiza automáticamente.
  

  // Guarda el nombre que escribe el usuario al registrarse
  const [nombre, setNombre] = useState('')

  // Controla en qué página estamos: 'inicio', 'lecciones', 'practica', 'perfil'
  const [pagina, setPagina] = useState('inicio')

  // Guarda el progreso del usuario (puntos y lecciones completadas)
  const [progreso, setProgreso] = useState({
    puntos: 0,
    leccionesCompletadas: 0,
    certificado: false
  })

  // Guarda si el usuario ya está registrado o no
  const [registrado, setRegistrado] = useState(false)

  // Guarda la seña que detectó la cámara
  const [senaDetectada, setSenaDetectada] = useState('')

  // Guarda si la seña fue correcta o incorrecta
  const [resultado, setResultado] = useState('')

  // Referencia al elemento de video de la cámara
  const videoRef = useRef(null)

  // Lección actual que está practicando el usuario
  const [leccionActual, setLeccionActual] = useState(null)

  
  // LISTA DE LECCIONES
  // Cada lección tiene un ID, título, descripción y
  // descripción de cómo hacer la seña con los dedos.
  
  const lecciones = [
    {
      id: 1,
      titulo: 'Letra A',
      descripcion: 'Aprende a hacer la letra A en Lengua de Señas Mexicana',
      instruccion: 'Cierra el puño con el pulgar al lado'
    },
    {
      id: 2,
      titulo: 'Letra B',
      descripcion: 'Aprende a hacer la letra B en Lengua de Señas Mexicana',
      instruccion: 'Extiende los 4 dedos juntos, dobla el pulgar'
    },
    {
      id: 3,
      titulo: 'Letra C',
      descripcion: 'Aprende a hacer la letra C en Lengua de Señas Mexicana',
      instruccion: 'Forma una C con todos los dedos curvados'
    },
    {
      id: 4,
      titulo: 'Letra D',
      descripcion: 'Aprende a hacer la letra D en Lengua de Señas Mexicana',
      instruccion: 'Índice apuntando arriba, demás dedos tocando el pulgar'
    },
    {
      id: 5,
      titulo: 'Letra E',
      descripcion: 'Aprende a hacer la letra E en Lengua de Señas Mexicana',
      instruccion: 'Dobla todos los dedos hacia la palma'
    },
  ]

  
  // FUNCIÓN: Registrar usuario
  // Se llama cuando el usuario hace clic en "Registrarse"
  
  const handleRegistro = () => {
    // Verificamos que haya escrito su nombre
    if (!nombre.trim()) {
      alert('Por favor escribe tu nombre')
      return
    }
    // Marcamos que el usuario ya está registrado
    setRegistrado(true)
    // Lo llevamos a la página de lecciones
    setPagina('lecciones')
  }

  
  // FUNCIÓN: Iniciar práctica de una lección
  // Se llama cuando el usuario hace clic en "Practicar"

  const handlePracticar = (leccion) => {
    // Guardamos qué lección va a practicar
    setLeccionActual(leccion)
    // Limpiamos el resultado anterior
    setResultado('')
    setSenaDetectada('')
    // Lo llevamos a la página de práctica con la cámara
    setPagina('practica')
  }

  
  // FUNCIÓN: Simular detección de seña
  // Por ahora simula que la cámara detectó la seña correcta.
  // Más adelante aquí conectaremos MediaPipe.
  
  const handleDetectar = () => {
    // Simulamos que la seña fue detectada correctamente
    setSenaDetectada(leccionActual.titulo)
    setResultado('correcto')

    // Sumamos 10 puntos al progreso del usuario
    setProgreso(prev => ({
      ...prev,
      puntos: prev.puntos + 10,
      leccionesCompletadas: prev.leccionesCompletadas + 1
    }))
  }

  
  // FUNCIÓN: Reclamar certificado
  // Solo funciona si el usuario tiene 100 o más puntos

  const handleCertificado = () => {
    if (progreso.puntos >= 100) {
      setProgreso(prev => ({ ...prev, certificado: true }))
      alert('¡Felicidades! Tu certificado ha sido emitido en la blockchain.')
    } else {
      alert(`Te faltan ${100 - progreso.puntos} puntos para obtener tu certificado.`)
    }
  }

  
  // LO QUE SE MUESTRA EN PANTALLA
  // Dependiendo de la página actual, mostramos contenido diferente

  return (
    <div className="app">

      {/* BARRA DE NAVEGACIÓN - siempre visible */}
      <nav className="navbar">
        <h1 className="logo"> LSM Aprende</h1>
        {registrado && (
          <div className="nav-links">
            {/* Botones para navegar entre páginas */}
            <button onClick={() => setPagina('lecciones')}>Lecciones</button>
            <button onClick={() => setPagina('perfil')}>
              Mi perfil ({progreso.puntos} pts)
            </button>
          </div>
        )}
      </nav>

      {/* PÁGINA DE INICIO - aparece cuando pagina === 'inicio' */}
      {pagina === 'inicio' && (
        <div className="pagina-inicio">
          <h2>Bienvenida a LSM Aprende </h2>
          <p>Aprende Lengua de Señas Mexicana de forma interactiva con tu cámara.</p>

          {/* Formulario de registro */}
          <div className="registro">
            <input
              type="text"
              placeholder="¿Cómo te llamas?"
              value={nombre}
              // Cada vez que el usuario escribe, actualizamos el estado "nombre"
              onChange={(e) => setNombre(e.target.value)}
            />
            <button onClick={handleRegistro}>Comenzar a aprender</button>
          </div>
        </div>
      )}

      {/* PÁGINA DE LECCIONES - lista de señas para aprender */}
      {pagina === 'lecciones' && (
        <div className="pagina-lecciones">
          <h2>Lecciones disponibles </h2>
          <p>Hola {nombre}, elige una lección para practicar:</p>

          {/* Mostramos cada lección como una tarjeta */}
          <div className="grid-lecciones">
            {lecciones.map((leccion) => (
              <div key={leccion.id} className="tarjeta-leccion">
                <h3>{leccion.titulo}</h3>
                <p>{leccion.descripcion}</p>
                <button onClick={() => handlePracticar(leccion)}>
                  Practicar 
                </button>
              </div>
            ))}
          </div>
        </div>
      )}

      {/* PÁGINA DE PRÁCTICA - aquí va la cámara */}
      {pagina === 'practica' && leccionActual && (
        <div className="pagina-practica">
          <h2>Practicando: {leccionActual.titulo} </h2>
          <p className="instruccion"> {leccionActual.instruccion}</p>

          {/* Componente de la cámara con detección de manos */}
<HandCamera
  onSenaDetectada={(sena) => {
    // Cuando la cámara detecta la seña correcta, simulamos completar la lección
    handleDetectar()
  }}
  senaObjetivo={leccionActual.titulo}
/>

          {/* Mostramos el resultado de la detección */}
          {resultado === 'correcto' && (
            <div className="resultado-correcto">
              Bien Detectamos: {senaDetectada} — +10 puntos
            </div>
          )}    

          {/* Botón para volver a las lecciones */}
          <button className="btn-volver" onClick={() => setPagina('lecciones')}>
            ← Volver a lecciones
          </button>
        </div>
      )}

      {/* PÁGINA DE PERFIL - muestra el progreso del usuario */}
      {pagina === 'perfil' && (
        <div className="pagina-perfil">
          <h2>Mi perfil </h2>
          <div className="perfil-info">
            <p>Nombre: <strong>{nombre}</strong></p>
            <p>Puntos: <strong>{progreso.puntos}</strong></p>
            <p>Lecciones completadas: <strong>{progreso.leccionesCompletadas}</strong></p>
            <p>Certificado: <strong>{progreso.certificado ? 'Obtenido' : 'Pendiente'}</strong></p>
          </div>

          {/* Barra de progreso hacia el certificado */}
          <div className="barra-progreso">
            <p>Progreso hacia certificado: {progreso.puntos}/100 puntos</p>
            <div className="barra-fondo">
              {/* El ancho de la barra cambia según los puntos */}
              <div
                className="barra-relleno"
                style={{ width: `${Math.min(progreso.puntos, 100)}%` }}
              />
            </div>
          </div>

          {/* Botón para reclamar el certificado */}
          <button
            className="btn-certificado"
            onClick={handleCertificado}
            // Desactivamos el botón si ya tiene el certificado
            disabled={progreso.certificado}
          >
            {progreso.certificado ? 'Certificado obtenido :D' : '🎓 Reclamar certificado'}
          </button>
        </div>
      )}

    </div>
  )
}

// Exportamos el componente para que otros archivos puedan usarlo
export default App
