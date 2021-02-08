document
  .getElementById("functionButton")
  .addEventListener("click", function () {
    window.__TAURI__.tauri.invoke({
      cmd: "executeCommand",
      argument: "Hello World",
    });
  });

document.getElementById("requestButton").addEventListener("click", function () {
  window.__TAURI__.tauri
    .promisified({
      cmd: "performRequest",
      data: "dummy data",
    })
    .then(function (response) {
      document.getElementById("response").innerHTML =
        typeof response === "object" ? JSON.stringify(response) : response;
    });
});
