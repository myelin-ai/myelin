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

export function readBlob (blob: Blob): Promise<ArrayBuffer> {
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
