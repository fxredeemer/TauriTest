document
  .getElementById("startExecution")
  .addEventListener("click", function () {
    var baudRate = Number(document.getElementById("baudRate").value);
    var port = document.getElementById("port").value;
    window.__TAURI__.tauri
    .promisified({
      cmd: "startCommunication",
      baud: baudRate,
      port: port
    })
    .then(function (response) {
      document.getElementById("connectionResult").innerHTML =
        typeof response === "object" ? JSON.stringify(response) : response;
    });
  });
  
window.onload = function (){
  window.__TAURI__.tauri
  .promisified({
    cmd: "getPortInfo",
  })
  .then(function (response) {
    document.getElementById("content").innerHTML =
      typeof response === "object" ? JSON.stringify(response) : response;
  });
}
