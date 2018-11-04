import { createWebsocket, readBlob } from './websocket'

const websocketUrl = `ws://${window.location.hostname}:6956`

Promise.all([
    createWebsocket(websocketUrl),
    import('../out/myelin_visualization_client'),
]).then(([websocket, wasm]) => {
    const canvas = document.getElementById('visualization') as HTMLCanvasElement
    const inputHandler = wasm.init(canvas)
    const onMessage = (event: MessageEvent) => {
        readBlob(event.data)
            .then((arrayBuffer) => {
                try {
                    inputHandler.on_message(new Uint8Array(arrayBuffer))
                } catch (e) {
                    console.error(e)
                    websocket.removeEventListener('message', onMessage)
                    websocket.close()
                }
            })
            .catch((error) => {
                console.error('Failed to read websocket message blob', error)
            })
    }

    websocket.addEventListener('message', onMessage)
}).catch((reason) => {
    console.error(reason)
    document.body.appendChild(document.createTextNode('Failed to initialize visualization'))
    const reasonElement = document.createElement('pre')
    reasonElement.innerText = reason
    document.body.appendChild(reasonElement)
})
