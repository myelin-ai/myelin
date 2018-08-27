const rust = import('../out/myelin_visualization')
rust.then(rust => {
    const canvas = document.getElementById('myelin-visualization') as HTMLCanvasElement
    const entryPoint = rust.EntryPoint.new(canvas)
    entryPoint.start()
})
