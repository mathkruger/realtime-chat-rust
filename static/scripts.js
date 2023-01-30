const messageFields = {
    username: document.querySelector("#username"),
    message: document.querySelector("#message"),
    room: "main",
};
const connectionStatus = document.querySelector("#connection-status");
const messagesList = document.querySelector("#messages-list");
const sendButton = document.querySelector("#send");

let events = null; 

const handleEvents = {
    onMessage: (ev) => {
        const message = JSON.parse(ev.data || "null");

        if (message) {
            messagesList.innerHTML += `
                <li>${message.username}: ${message.message}</li>
            `;
        }
    },

    onOpen: (ev) => {
        connectionStatus.innerHTML = "Connected";
    },

    onError: (ev) => {
        if (events) {
            events.close();
        }

        connectionStatus.innerHTML = "Disconnected, error maybe?"
    }
}

function subscribe(uri) {
    events = new EventSource(uri);

    events.addEventListener("message", handleEvents.onMessage);
    events.addEventListener("open", handleEvents.onOpen);
    events.addEventListener("error", handleEvents.onError);
}

async function sendMessage() {
    const message = {
        room: messageFields.room,
        username: messageFields.username.value,
        message: messageFields.message.value
    };

    await fetch("/message", {
        method: "POST",
        body: new URLSearchParams(message)
    });

    messageFields.message.value = "";
}

sendButton.addEventListener("click", async () => {
    await sendMessage();
});

subscribe("/events");