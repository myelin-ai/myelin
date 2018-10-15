const websocketUrl = `ws://${window.location.hostname}:6956`

function createWebsocket (url: string): Promise<WebSocket> {
    return new Promise((resolve, reject) => {
        const websocket = new WebSocket(url)

        const onError = () => {
            reject('Websocket failed to connect')
            removeListeners()
        }

        const onOpen = () => {
            resolve(websocket)
            removeListeners()
        }

        const removeListeners = () => {
            websocket.removeEventListener('error', onError)
            websocket.removeEventListener('open', onOpen)
        }

        websocket.addEventListener('error', onError)
        websocket.addEventListener('open', onOpen)
    })
}

function readBlob (blob: Blob): Promise<ArrayBuffer> {
    return new Promise((resolve, reject) => {
        const fileReader = new FileReader()

        const onError = () => {
            reject('Websocket failed to connect')
            removeListeners()
        }

        const onLoadEnd = () => {
            resolve(fileReader.result as ArrayBuffer)
            removeListeners()
        }

        const removeListeners = () => {
            fileReader.removeEventListener('error', onError)
            fileReader.removeEventListener('loadend', onLoadEnd)
        }

        fileReader.addEventListener('error', onError)
        fileReader.addEventListener('loadend', onLoadEnd)

        fileReader.readAsArrayBuffer(blob)
    })
}

Promise.all([
    createWebsocket(websocketUrl),
    import('../out/myelin_visualization_client'),
]).then(([websocket, wasm]) => {
    const canvas = document.getElementById('visualization') as HTMLCanvasElement
    const inputHandler = wasm.init(canvas)
    websocket.addEventListener('message', (event) => {
        readBlob(event.data)
            .then((arrayBuffer) => {
                inputHandler.on_message(new Uint8Array(arrayBuffer))
            })
    })
}).catch((reason) => {
    console.error(reason)
    document.body.appendChild(document.createTextNode('Failed to initialize visualization'))
    const reasonElement = document.createElement('pre')
    reasonElement.innerText = reason
    document.body.appendChild(reasonElement)
})
