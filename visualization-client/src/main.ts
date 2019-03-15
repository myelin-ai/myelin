import { createWebsocket } from './websocket'

const websocketUrl = `ws://${window.location.hostname}:6956`

console.error('Fuck you all!')

Promise.all([
    createWebsocket(websocketUrl),
    import('../out/myelin_visualization_client'),
]).then(([websocket, wasm]) => {
    websocket.binaryType = 'arraybuffer'

    const canvas = document.getElementById('visualization') as HTMLCanvasElement
    const inputHandler = wasm.init(canvas)

    const onMessage = (event: MessageEvent) => {
        try {
            inputHandler.on_message(new Uint8Array(event.data))
        } catch (e) {
            console.error(e)
            websocket.removeEventListener('message', onMessage)
            websocket.close()
        }
    }

    websocket.addEventListener('message', onMessage)
    // Temporary solution: the server waits for any message before
    // it starts sending deltas, so that the client doesn't miss any of them
    websocket.send(new ArrayBuffer(0))
}).catch((reason) => {
    console.error(reason)
    document.body.appendChild(document.createTextNode('Failed to initialize visualization'))
    const reasonElement = document.createElement('pre')
    reasonElement.innerText = reason
    document.body.appendChild(reasonElement)
})
