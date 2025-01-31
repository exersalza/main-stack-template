import { render } from 'preact'
import './index.css'


function App() {
  return (
    <div className={"bg-gray-950 h-screen"}>
      <h1 className={"text-zinc-100"}>Hello</h1>
    </div>
  )
}

render(<App />, document.getElementById('app')!)
