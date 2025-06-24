# GeoFS HD Free
 Use GeoFS HD for free with ads for those that don't want to bankrupt Geo-FS while still supporting the game. To load just upload like usual, the main script to put into console is script.js


## Instructions
1. Copy the text in:
 /userscript/script.js OR copy the text below the instructions.
2. Go to geofs.com via the address or Shadow proxy.
3. Press Ctrl+Shift+I to open the inspector.
4. Click the Console tab.
5. Spam click the cancel error button until all the errors disappear.
6. Type "allow pasting" in the editor.
7. Press Enter.
8. Press Ctrl+V to paste the script.
9. Press Enter again.
10. Enjoy GeoFS **free** High Definition!!

Bonus Tip: Enable an ad-blocker to disable ads as well!

-----------------------------------------------------------------------
## Code

    const provider = "google";
    const multiplayerServer = "default"

    window.geofsNewHDState = true;
    window.geofs.geoIpUpdate = function() {
        delete window.geofs.api.analytics;
        document.body.classList.add("geofs-hd");

        if (multiplayerServer !== "default") {
            window.geofs.multiplayerHost = multiplayerServer;
        }

        switch (provider) {
            case "cache":
                window.geofs.api.imageryProvider = new window.Cesium.UrlTemplateImageryProvider({
                    maximumLevel: 21,
                    hasAlphaChannel: false,
                    subdomains: "abcdefghijklmnopqrstuvwxyz".split(""),
                    url: "http://localhost/map/{z}/{x}/{y}"
                });
                break;
            case "google":
                window.geofs.api.imageryProvider = new window.Cesium.UrlTemplateImageryProvider({
                    maximumLevel: 21,
                    hasAlphaChannel: false,
                    subdomains: ["mt0", "mt1", "mt2", "mt3"],
                    url: "https://{s}.google.com/vt/lyrs=s&x={x}&y={y}&z={z}"
                });
                break;
            case "apple":
                window.geofs.api.imageryProvider = new window.Cesium.UrlTemplateImageryProvider({
                    maximumLevel: 21,
                    hasAlphaChannel: false,
                    subdomains: ["sat-cdn1", "sat-cdn2", "sat-cdn3", "sat-cdn4"],
                    url: "https://{s}.apple-mapkit.com/tile?style=7&size=1&scale=1&z={z}&x={x}&y={y}&v=9651&accessKey=1705988638_4603104305979553294_%2F_Qvq1XXyXG5w0IUYlFOsIQsxLt2ALxm32i%2BAMbLIFD5s%3D"
                });
                break;
            case "bing":
                window.geofs.api.imageryProvider = new window.Cesium.BingMapsImageryProvider({
                    url: "https://dev.virtualearth.net",
                    key: "AjrgR5TNicgFReuFwvNH71v4YeQNkXIB20l63ZMm86mVuBGZPhTHMkdiVq2_9L7x",
                    mapStyle: window.Cesium.BingMapsStyle.AERIAL
                });
                break;
            default: break
        }

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

    //document.querySelectorAll("body > div.geofs-adbanner.geofs-adsense-container")[0].remove();

-----------------------------------------------------------------------------------------------------------------

Updated for GeoFS 2025 by flikrheist
