document
  .getElementById("startExecution")
  .addEventListener("click", function () {
    var address = document.getElementById("address").value;
    window.__TAURI__.tauri
      .promisified({
        cmd: "startCommunication",
        address: address,
      })
      .then(function (response) {
        document.getElementById("content").innerHTML =
          typeof response === "object" ? JSON.stringify(response) : response;
      });
  });