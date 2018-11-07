export function createWebsocket (url: string): Promise<WebSocket> {
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
