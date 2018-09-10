import('../out/myelin_visualization').then((wasm) => {
    const canvas = document.getElementById('visualization') as HTMLCanvasElement
    const inputHandler = wasm.init(canvas)
    inputHandler.on_timer()
}).catch((reason) => {
    console.error(reason)
    document.body.appendChild(document.createTextNode('Failed to load WASM'))
    const reasonElement = document.createElement('pre')
    reasonElement.innerText = reason
    document.body.appendChild(reasonElement)
})
