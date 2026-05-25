import { useState, useEffect } from 'react'
import init, { greet, expensive_calculation } from 'rust-wasm-lib'
import './App.css'

function App() {
  const [name, setName] = useState('Developer')
  const [greeting, setGreeting] = useState('')
  const [mathResult, setMathResult] = useState<number | null>(null)
  const [isWasmLoaded, setIsWasmLoaded] = useState(false)

  useEffect(() => {
    // Initialize WASM on mount
    init().then(() => {
      setIsWasmLoaded(true)
      console.log('WASM Loaded successfully')
    }).catch(err => {
      console.error('Failed to load WASM:', err)
    })
  }, [])

  const handleGreet = () => {
    if (isWasmLoaded) {
      setGreeting(greet(name))
    }
  }

  const handleCalculate = () => {
    if (isWasmLoaded) {
      // Calling Rust's expensive_calculation(a, b)
      setMathResult(expensive_calculation(10, 5))
    }
  }

  return (
    <div className="App" style={{ padding: '2rem', textAlign: 'center' }}>
      <h1>React + Rust WASM</h1>
      
      {!isWasmLoaded ? (
        <p>Loading WASM...</p>
      ) : (
        <div style={{ display: 'flex', flexDirection: 'column', gap: '1rem', alignItems: 'center' }}>
          <section style={{ border: '1px solid #ccc', padding: '1rem', borderRadius: '8px', width: '300px' }}>
            <h3>Greeting Service</h3>
            <input 
              value={name} 
              onChange={(e) => setName(e.target.value)}
              placeholder="Enter your name"
              style={{ padding: '0.5rem', marginBottom: '0.5rem' }}
            />
            <br />
            <button onClick={handleGreet}>Greet me from Rust!</button>
            <p><strong>{greeting}</strong></p>
          </section>

          <section style={{ border: '1px solid #ccc', padding: '1rem', borderRadius: '8px', width: '300px' }}>
            <h3>Math Service</h3>
            <p>Calculating <code>10 * 5 + 42</code> in Rust</p>
            <button onClick={handleCalculate}>Run Calculation</button>
            {mathResult !== null && (
              <p>Result: <strong>{mathResult}</strong></p>
            )}
          </section>
        </div>
      )}
    </div>
  )
}

export default App
