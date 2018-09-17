import('../out/myelin_visualization').then((wasm) => {
    const canvas = document.getElementById('visualization') as HTMLCanvasElement
    const inputHandler = wasm.init(canvas)
    const MILLIS_IN_SECOND = 1000
    const simulatedTimestep = wasm.simulated_timestep() * MILLIS_IN_SECOND
    const onTimer = () => inputHandler.on_timer()
    onTimer()
    setInterval(onTimer, simulatedTimestep)
}).catch((reason) => {
    console.error(reason)
    document.body.appendChild(document.createTextNode('Failed to load WASM'))
    const reasonElement = document.createElement('pre')
    reasonElement.innerText = reason
    document.body.appendChild(reasonElement)
})
