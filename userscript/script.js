// ==UserScript==
// @name         Better Resolution Terrain
// @namespace    http://tampermonkey.net/
// @version      2024-01-21
// @description  Gets higher resolution images
// @author       You
// @match        https://www.geo-fs.com/geofs.php?v=*
// @icon         https://www.google.com/s2/favicons?sz=64&domain=geo-fs.com
// @grant        none
// ==/UserScript==

(function() {
    "use strict";

    const mapsServers = ["mt0", "mt1", "mt2", "mt3"];

    window.geofsNewHDState = true;
    window.geofs.geoIpUpdate = function() {
        document.body.classList.add("geofs-hd");
        //window.geofs.api.imageryProvider = new window.Cesium.BingMapsImageryProvider({
        //    url: "https://dev.virtualearth.net",
        //    key: "AjrgR5TNicgFReuFwvNH71v4YeQNkXIB20l63ZMm86mVuBGZPhTHMkdiVq2_9L7x",
        //    mapStyle: window.Cesium.BingMapsStyle.AERIAL
        //});
        //url: "http://localhost/map/{z}/{x}/{y}.jpg"
        //window.geofs.api.imageryProvider = new window.Cesium.UrlTemplateImageryProvider({
        //    maximumLevel: 21,
        //    customTags : {
        //        server: function(imageryProvider, x, y, level) {
        //            return mapsServers[Math.floor(Math.random() * mapsServers.length)];
        //        }
        //    },
        //    url: "https://{server}.google.com/vt/lyrs=s&x={x}&y={y}&z={z}"
        //});
        window.geofs.api.imageryProvider = new window.Cesium.UrlTemplateImageryProvider({
            maximumLevel: 21,
            url: "http://localhost/map/{z}/{x}/{y}"
        });
        window.geofs.api.setImageryProvider(window.geofs.api.imageryProvider, false);
        window.geofs.api.viewer.terrainProvider = window.geofs.api.flatRunwayTerrainProviderInstance = new window.geofs.api.FlatRunwayTerrainProvider({
            baseProvider: new window.Cesium.CesiumTerrainProvider({
                url: "https://data.geo-fs.com/srtm/",
                requestWaterMask: false,
                requestVertexNormals: true
            }),
            bypass: false,
            maximumLevel: 12
        });
    };
    window.executeOnEventDone("geofsStarted", function() {
        if (window.geofs.api.hdOn === window.geofsNewHDState) return;
        window.jQuery("body").trigger("terrainProviderWillUpdate");
        window.geofs.geoIpUpdate();
        window.geofs.api.hdOn = window.geofsNewHDState;
        window.geofs.api.renderingQuality();
        window.jQuery("body").trigger("terrainProviderUpdate");
    });
    window.executeOnEventDone("afterDeferredload", function() {
        window.geofs.mapXYZ = "https://data.geo-fs.com/osm/{z}/{x}/{y}.png";
    });

    document.querySelectorAll("body > div.geofs-adbanner.geofs-adsense-container")[0].remove();
})();