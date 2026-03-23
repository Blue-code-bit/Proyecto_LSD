// Importamos las herramientas de React que necesitamos
import { useState, useRef } from 'react'

// Importamos los estilos de la página
import './App.css'

// Importamos el componente de la cámara
import HandCamera from './HandCamera'

function App() {

  
  // VARIABLES DE ESTADO
  

  // Nombre del usuario
  const [nombre, setNombre] = useState('')

  // Página actual: 'inicio', 'modulos', 'lecciones', 'practica', 'perfil'
  const [pagina, setPagina] = useState('inicio')

  // Módulo seleccionado actualmente
  const [moduloActual, setModuloActual] = useState(null)

  // Lección actual que está practicando
  const [leccionActual, setLeccionActual] = useState(null)

  // Progreso del usuario
  const [progreso, setProgreso] = useState({
    puntos: 0,
    leccionesCompletadas: 0,
    certificado: false
  })

  // Si el usuario ya está registrado
  const [registrado, setRegistrado] = useState(false)

  // Resultado de la detección
  const [resultado, setResultado] = useState('')
  const [senaDetectada, setSenaDetectada] = useState('')

  
  // MÓDULOS TEMÁTICOS
  // Cada módulo tiene un ícono, título, descripción, color y lecciones
  
  const modulos = [
    {
      id: 'abecedario',
      icono: '🔤',
      titulo: 'Abecedario LSM',
      descripcion: 'Aprende las 27 letras del alfabeto en Lengua de Señas Mexicana',
      color: '#9945ff',
      lecciones: [
        { id: 1, titulo: 'Letra A', instruccion: 'Cierra el puño con el pulgar al lado' },
        { id: 2, titulo: 'Letra B', instruccion: 'Extiende los 4 dedos juntos, dobla el pulgar' },
        { id: 3, titulo: 'Letra C', instruccion: 'Forma una C con todos los dedos curvados' },
        { id: 4, titulo: 'Letra D', instruccion: 'Índice apuntando arriba, demás dedos tocando el pulgar' },
        { id: 5, titulo: 'Letra E', instruccion: 'Dobla todos los dedos hacia la palma' },
      ]
    },
    {
      id: 'saludos',
      icono: '👋',
      titulo: 'Saludos',
      descripcion: 'Aprende a saludar y despedirte en Lengua de Señas Mexicana',
      color: '#14f195',
      lecciones: [
        { id: 6, titulo: 'Hola', instruccion: 'Agita la mano abierta de lado a lado' },
        { id: 7, titulo: 'Adiós', instruccion: 'Extiende la mano y dobla los dedos hacia abajo repetidamente' },
        { id: 8, titulo: 'Por favor', instruccion: 'Mano abierta en el pecho, mueve en círculos' },
        { id: 9, titulo: 'Gracias', instruccion: 'Toca los labios con la mano y extiéndela hacia adelante' },
      ]
    },
    {
      id: 'tiempo',
      icono: '🌅',
      titulo: 'Buenos días / tardes / noches',
      descripcion: 'Aprende a expresar los saludos del día en LSM',
      color: '#f59e0b',
      lecciones: [
        { id: 10, titulo: 'Buenos días', instruccion: 'Señala hacia arriba con ambas manos abiertas' },
        { id: 11, titulo: 'Buenas tardes', instruccion: 'Mano horizontal moviéndose hacia abajo lentamente' },
        { id: 12, titulo: 'Buenas noches', instruccion: 'Manos cruzadas frente al pecho, inclina la cabeza' },
        { id: 13, titulo: 'Hasta mañana', instruccion: 'Señala hacia adelante y luego cierra la mano' },
      ]
    },
    {
      
  id: 'groserias',
  icono: '🤬',
  titulo: 'Groserías',
  descripcion: 'Aprende las palabras coloquiales y groserías más comunes en LSM',
  color: '#ef4444',
  lecciones: [
    { id: 14, titulo: 'Wey', instruccion: 'Señala con el índice y mueve la mano de lado a lado' },
    { id: 15, titulo: 'Tonto', instruccion: 'Toca la sien con el índice y gira levemente' },
    { id: 16, titulo: 'Metiche', instruccion: 'Índice apuntando hacia adelante moviéndose repetidamente' },
    { id: 17, titulo: 'Exagerado', instruccion: 'Ambas manos abiertas moviéndose hacia afuera exageradamente' },
    { id: 18, titulo: 'Flojo', instruccion: 'Mano caída hacia abajo con muñeca suelta' },
  { id: 15, titulo: 'Naco', instruccion: 'Pulgar hacia abajo con expresión de desaprobación' },
  ]
,
    }
  ]

  
  // FUNCIONES

  // Registrar usuario
  const handleRegistro = () => {
    if (!nombre.trim()) {
      alert('Por favor escribe tu nombre')
      return
    }
    setRegistrado(true)
    setPagina('modulos')
  }

  // Entrar a un módulo
  const handleModulo = (modulo) => {
    setModuloActual(modulo)
    setPagina('lecciones')
  }

  // Practicar una lección
  const handlePracticar = (leccion) => {
    setLeccionActual(leccion)
    setResultado('')
    setSenaDetectada('')
    setPagina('practica')
  }

  // Cuando la cámara detecta una seña correcta
  const handleDetectar = () => {
    setSenaDetectada(leccionActual.titulo)
    setResultado('correcto')
    setProgreso(prev => ({
      ...prev,
      puntos: prev.puntos + 10,
      leccionesCompletadas: prev.leccionesCompletadas + 1
    }))
  }

  // Reclamar certificado
  const handleCertificado = () => {
    if (progreso.puntos >= 100) {
      setProgreso(prev => ({ ...prev, certificado: true }))
      alert('¡Felicidades! Tu certificado ha sido emitido en la blockchain.')
    } else {
      alert(`Te faltan ${100 - progreso.puntos} puntos para obtener tu certificado.`)
    }
  }


  // PANTALLA

  return (
    <div className="app">

      {/* BARRA DE NAVEGACIÓN */}
      <nav className="navbar">
        <h1 className="logo" onClick={() => registrado && setPagina('modulos')} style={{cursor: registrado ? 'pointer' : 'default'}}>
           LSM APPRENDE
        </h1>
        {registrado && (
          <div className="nav-links">
            <button onClick={() => setPagina('modulos')}>Módulos</button>
            <button onClick={() => setPagina('perfil')}>
              Mi perfil ({progreso.puntos} pts)
            </button>
          </div>
        )}
      </nav>

      {/* PÁGINA DE INICIO */}
      {pagina === 'inicio' && (
        <div className="pagina-inicio">
          <div className="inicio-hero">
            <h2>Aprende LSM con tu cámara </h2>
            <p>Lengua de Señas Mexicana de forma interactiva, con detección de manos en tiempo real y progreso guardado en blockchain.</p>
          </div>
          <div className="registro">
            <input
              type="text"
              placeholder="¿Cómo te llamas?"
              value={nombre}
              onChange={(e) => setNombre(e.target.value)}
              onKeyDown={(e) => e.key === 'Enter' && handleRegistro()}
            />
            <button onClick={handleRegistro}>Comenzar a aprender →</button>
          </div>
        </div>
      )}

      {/* PÁGINA DE MÓDULOS */}
      {pagina === 'modulos' && (
        <div className="pagina-modulos">
          <h2>Hola {nombre} ¿Qué quieres aprender hoy?</h2>
          <p className="subtitulo">Elige un módulo para comenzar</p>

          {/* Grid de módulos */}
          <div className="grid-modulos">
            {modulos.map((modulo) => (
              <div
                key={modulo.id}
                className="tarjeta-modulo"
                style={{ borderColor: modulo.color }}
                onClick={() => handleModulo(modulo)}
              >
                {/* Ícono grande del módulo */}
                <div className="modulo-icono" style={{ backgroundColor: modulo.color + '20' }}>
                  <span>{modulo.icono}</span>
                </div>

                {/* Info del módulo */}
                <div className="modulo-info">
                  <h3 style={{ color: modulo.color }}>{modulo.titulo}</h3>
                  <p>{modulo.descripcion}</p>
                  <span className="modulo-lecciones">{modulo.lecciones.length} lecciones</span>
                </div>

                {/* Flecha */}
                <div className="modulo-flecha" style={{ color: modulo.color }}>→</div>
              </div>
            ))}
          </div>
        </div>
      )}

      {/* PÁGINA DE LECCIONES DE UN MÓDULO */}
      {pagina === 'lecciones' && moduloActual && (
        <div className="pagina-lecciones">

          {/* Header del módulo */}
          <div className="lecciones-header" style={{ borderColor: moduloActual.color }}>
            <button className="btn-volver" onClick={() => setPagina('modulos')}>← Volver</button>
            <h2 style={{ color: moduloActual.color }}>
              {moduloActual.icono} {moduloActual.titulo}
            </h2>
            <p>{moduloActual.descripcion}</p>
          </div>

          {/* Grid de lecciones */}
          <div className="grid-lecciones">
            {moduloActual.lecciones.map((leccion) => (
              <div key={leccion.id} className="tarjeta-leccion" style={{ borderColor: moduloActual.color + '60' }}>
                <h3>{leccion.titulo}</h3>
                <p>{leccion.instruccion}</p>
                <button
                  style={{ backgroundColor: moduloActual.color }}
                  onClick={() => handlePracticar(leccion)}
                >
                  Practicar 📷
                </button>
              </div>
            ))}
          </div>
        </div>
      )}

      {/* PÁGINA DE PRÁCTICA CON CÁMARA */}
      {pagina === 'practica' && leccionActual && (
        <div className="pagina-practica">
          <button className="btn-volver" onClick={() => setPagina('lecciones')}>← Volver</button>
          <h2>Practicando: {leccionActual.titulo} </h2>
          <p className="instruccion">{leccionActual.instruccion}</p>

          {/* Componente de cámara con detección */}
          <HandCamera
            onSenaDetectada={(sena) => handleDetectar()}
            senaObjetivo={leccionActual.titulo}
          />

          {/* Resultado de la detección */}
          {resultado === 'correcto' && (
            <div className="resultado-correcto">
              Correcto :D Detectamos: {senaDetectada} — +10 puntos
            </div>
          )}
        </div>
      )}

      {/* PÁGINA DE PERFIL */}
      {pagina === 'perfil' && (
        <div className="pagina-perfil">
          <h2>Mi perfil 👤</h2>
          <div className="perfil-info">
            <p> Nombre: <strong>{nombre}</strong></p>
            <p>Puntos: <strong>{progreso.puntos}</strong></p>
            <p>Lecciones completadas: <strong>{progreso.leccionesCompletadas}</strong></p>
            <p>Certificado: <strong>{progreso.certificado ? '¡Obtenido!' : 'Pendiente'}</strong></p>
          </div>

          {/* Barra de progreso */}
          <div className="barra-progreso">
            <p>Progreso hacia certificado: {progreso.puntos}/100 puntos</p>
            <div className="barra-fondo">
              <div
                className="barra-relleno"
                style={{ width: `${Math.min(progreso.puntos, 100)}%` }}
              />
            </div>
          </div>

          <button
            className="btn-certificado"
            onClick={handleCertificado}
            disabled={progreso.certificado}
          >
            {progreso.certificado ? '🏆 Certificado obtenido' : '🎓 Reclamar certificado'}
          </button>
        </div>
      )}

    </div>
  )
}

export default App