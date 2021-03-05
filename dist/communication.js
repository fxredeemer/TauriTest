document
    .getElementById("connect")
    .addEventListener("click", function () {
        var address = document.getElementById("address").value;
        window.__TAURI__.tauri
            .promisified({
                cmd: "connect",
                address: address,
            })
            .then(function (response) {
                console.log(response);
                if(response === "Success"){
                    document.getElementById("statusmessage").innerHTML = "Connected";
                }
                else {
                    document.getElementById("statusmessage").innerHTML = "Connection Error";
                }
            });
    });


window.__TAURI__.event
    .listen("rust-event", function (response) {
        var list = document.getElementById('messages');
        var entry = document.createElement('li');

        var data = response.payload.data;

        entry.appendChild(document.createTextNode(data));
        list.appendChild(entry);
    });