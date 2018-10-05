import('../out/myelin_visualization_client').then((wasm) => {
    const canvas = document.getElementById('visualization') as HTMLCanvasElement
    wasm.init(canvas)
}).catch((reason) => {
    console.error(reason)
    document.body.appendChild(document.createTextNode('Failed to load WASM'))
    const reasonElement = document.createElement('pre')
    reasonElement.innerText = reason
    document.body.appendChild(reasonElement)
})
