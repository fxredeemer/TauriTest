import tauri from '../node_modules/tauri/api/tauri';
import event from '../node_modules/tauri/api/event';

document
    .getElementById("connect")
    .addEventListener("click", function () {
        var address = document.getElementById("address").value;
        tauri
            .promisified({
                cmd: "connect",
                address: address,
            })
            .then(function (response) {
                console.log(response);
                if (response === "Success") {
                    document.getElementById("statusmessage").innerHTML = "Connected";
                }
                else {
                    document.getElementById("statusmessage").innerHTML = "Connection Error";
                }
            });
    });

document
    .getElementById("setup")
    .addEventListener("click", function () {
        tauri.invoke({
            cmd: "setup",
            payload: "command payload",
        })
    })


event
    .listen("rust-event", function (response) {
        var list = document.getElementById('messages');
        var entry = document.createElement('li');

        var data = response.payload.data;

        entry.appendChild(document.createTextNode(data));
        list.appendChild(entry);
    });