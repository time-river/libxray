package main

import "C"

import (
	"encoding/json"
	"os"
	"os/signal"
	"runtime/debug"
	"syscall"

	"github.com/xtls/xray-core/core"
	"github.com/xtls/xray-core/infra/conf"
	_ "github.com/xtls/xray-core/main/distro/all"
)

var (
	coreServer *core.Instance
)

//export xrayVersion
func xrayVersion() *C.char {
	ver := core.Version()
	return C.CString(ver)
}

// TODO: detect unused port automatically
func setupServer(data []byte) (*core.Instance, error) {
	conf := &conf.Config{}

	err := json.Unmarshal(data, conf)
	if err != nil {
		return nil, err
	}

	cf, err := conf.Build()
	if err != nil {
		return nil, err
	}

	server, err := core.New(cf)
	if err != nil {
		return nil, err
	}

	return server, nil
}

func setMaxMemory(max int64) {
	debug.SetGCPercent(10)
	debug.SetMemoryLimit(max)
}

func checkDir(path string) bool {
	stat, err := os.Stat(path)
	return err == nil && stat.IsDir()
}

// If `xrayStart` args is `string`, it's okay on Linux platform, but has problem on Windows.
// Windows will report error: runtime: out of memory: cannot allocate xxx-byte block
//
//export xrayStart
func xrayStart(cconf *C.char, cdir *C.char, maxMem C.long) *C.char {
	if cdir != nil {
		dir := C.GoString(cdir)
		if checkDir(dir) {
			os.Setenv("xray.location.asset", dir)
		}
	}

	if maxMem > 0 {
		setMaxMemory(int64(maxMem))
	}

	conf := C.GoString(cconf)
	println(">>> ", conf)
	coreServer, err := setupServer([]byte(conf))
	if err != nil {
		estr := err.Error()
		return C.CString(estr)
	}

	// TODO: timeout check
	// It won't start success on Windows platform
	// if the port is occupied
	if err := coreServer.Start(); err != nil {
		estr := err.Error()
		return C.CString(estr)
	}

	debug.FreeOSMemory()
	return nil
}

//export xrayStop
func xrayStop() *C.char {
	if coreServer == nil {
		return nil
	}

	err := coreServer.Close()
	coreServer = nil
	if err != nil {
		estr := err.Error()
		return C.CString(estr)
	}

	return nil
}

// test
func main() {
	conf := `{
		"inbounds": [
		  {
			"port": 1080,
			"protocol": "socks",
			"enabled": true,
			"settings": {
			  "udp": true,
			  "auth": "noauth"
			},
			"listen": "127.0.0.1"
		  }
		]
	  }
	`

	cconf := C.CString(conf)
	xrayStart(cconf, nil, 0)

	{
		osSignals := make(chan os.Signal, 1)
		signal.Notify(osSignals, os.Interrupt, syscall.SIGTERM)
		<-osSignals
	}
}
