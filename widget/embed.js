(function () {
  var script = document.currentScript;
  var city = (script && script.getAttribute("data-city")) || "Ahmedabad";
  var api = (script && script.getAttribute("data-api")) || "http://127.0.0.1:8088";
  var host = document.createElement("div");
  host.setAttribute("data-sthan-panchang", city);
  host.style.cssText =
    "font:14px/1.4 system-ui,sans-serif;max-width:280px;padding:12px 14px;border:1px solid #ddd;border-radius:8px;background:#fff8f0;color:#222";
  host.textContent = "Loading panchang…";
  script.parentNode.insertBefore(host, script.nextSibling);

  fetch(api + "/v1/panchang?city=" + encodeURIComponent(city))
    .then(function (r) { return r.json(); })
    .then(function (d) {
      var place = (d.place && d.place.name) || city;
      host.innerHTML =
        "<strong>" + place + "</strong><br>" +
        "Tithi " + d.tithi_number + " " + d.paksha + "<br>" +
        "Sunrise " + (d.sunrise || "").slice(11, 16) + " UTC";
    })
    .catch(function () {
      host.textContent = "Panchang unavailable";
    });
})();
