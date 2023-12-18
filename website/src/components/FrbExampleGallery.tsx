import React from 'react';

// noinspection JSUnusedGlobalSymbols
export default function FrbExampleGallery() {
    return (
        <>
            <div id="flutter_host" style={{height: '500px', border: '1px solid #ccc'}}>Loading Flutter web app...
            </div>
            <FlutterInitializer/>
        </>
    )
}

class FlutterInitializer extends React.Component {
    componentDidMount() {
        console.log('FrbExampleGallery.FlutterInitializer componentDidMount');

        {
            const script = document.createElement("script");
            script.src = 'enable-threads.js';
            document.body.appendChild(script);
        }

        {
            const script = document.createElement("script");
            script.src = 'flutter.js';
            // script.defer = true; // TODO
            document.body.appendChild(script);
        }

        window.addEventListener("load", function (ev) {
            console.log('FrbExampleGallery.FlutterInitializer call flutter-loadEntrypoint');
            _flutter.loader.loadEntrypoint({
                onEntrypointLoaded: async function (engineInitializer) {
                    console.log('FrbExampleGallery.FlutterInitializer call flutter-appRunner');
                    let appRunner = await engineInitializer.initializeEngine({
                        // Pass a reference to "div#flutter_host" into the Flutter engine.
                        hostElement: document.querySelector("#flutter_host")
                    });
                    await appRunner.runApp();
                }
            });
        });
    }

    render() {
        return <span></span>
    }
}

