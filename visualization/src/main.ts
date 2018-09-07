import('../out/myelin_visualization').then((rust) => {
    const canvas = document.getElementById('myelin-visualization') as HTMLCanvasElement
    const entryPoint = rust.EntryPoint.new(canvas)
    entryPoint.start();
}).catch((reason) => {
    console.error(reason)
    document.body.appendChild(document.createTextNode('Failed to load WASM'))
    const reasonElement = document.createElement('pre')
    reasonElement.innerText = reason
    document.body.appendChild(reasonElement)
})
