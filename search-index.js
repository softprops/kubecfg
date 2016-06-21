var searchIndex = {};
searchIndex["kubecfg"] = {"doc":"","items":[[3,"Context","kubecfg","Represents a kubernetes cluster and namespace authentication",null,null],[12,"cluster","","The name of a cluster",0,null],[12,"namespace","","The name of a namespace",0,null],[12,"user","","The name of a user",0,null],[3,"Cluster","","Describes information needed to resolve\na connection to a cluster",null,null],[12,"api_version","","The clusters supported api version",1,null],[12,"server","","The server URI",1,null],[12,"insecure_skip_tls_verify","","Predicate used to determine if a client should skip tls verification",1,null],[12,"certificate_authority","","Content used by client to certify the server is authentic",1,null],[3,"User","","User authentication credentials\nto authenticate requests to a kubernetes cluster",null,null],[12,"client_certificate","","",2,null],[12,"client_key","","",2,null],[12,"token","","",2,null],[12,"username","","",2,null],[12,"password","","",2,null],[3,"Config","","Represents local kubernetes configuration settings",null,null],[12,"clusters","","A map of cluster name to cluster",3,null],[12,"contexts","","A map of context name to context",3,null],[12,"users","","A map of user name to user",3,null],[12,"current_context","","The current context&#39;s name",3,null],[4,"Error","","Encapsulation of potential errors\nthat may happen when resolving\na kubernetets config",null,null],[13,"Homeless","","A failure to resolve a home directory",4,null],[13,"IO","","IO errors",4,null],[13,"Yaml","","Failure to parse yaml data",4,null],[4,"Content","","Represents a way to resolve content",null,null],[13,"Path","","Location of content on disk",5,null],[13,"Data","","Raw content string",5,null],[6,"Result","","A type alias for the result operations which may return an `kubecfg::Error`",null,null],[11,"fmt","","",4,{"inputs":[{"name":"error"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"from","","",4,{"inputs":[{"name":"ioerror"}],"output":{"name":"error"}}],[11,"from","","",4,{"inputs":[{"name":"scanerror"}],"output":{"name":"error"}}],[11,"eq","","",0,{"inputs":[{"name":"context"},{"name":"context"}],"output":{"name":"bool"}}],[11,"ne","","",0,{"inputs":[{"name":"context"},{"name":"context"}],"output":{"name":"bool"}}],[11,"fmt","","",0,{"inputs":[{"name":"context"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"new","","",0,{"inputs":[{"name":"option"},{"name":"option"},{"name":"option"}],"output":{"name":"context"}}],[11,"eq","","",5,{"inputs":[{"name":"content"},{"name":"content"}],"output":{"name":"bool"}}],[11,"ne","","",5,{"inputs":[{"name":"content"},{"name":"content"}],"output":{"name":"bool"}}],[11,"fmt","","",5,{"inputs":[{"name":"content"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"eq","","",1,{"inputs":[{"name":"cluster"},{"name":"cluster"}],"output":{"name":"bool"}}],[11,"ne","","",1,{"inputs":[{"name":"cluster"},{"name":"cluster"}],"output":{"name":"bool"}}],[11,"fmt","","",1,{"inputs":[{"name":"cluster"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"eq","","",2,{"inputs":[{"name":"user"},{"name":"user"}],"output":{"name":"bool"}}],[11,"ne","","",2,{"inputs":[{"name":"user"},{"name":"user"}],"output":{"name":"bool"}}],[11,"fmt","","",2,{"inputs":[{"name":"user"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"new","","",2,{"inputs":[{"name":"option"},{"name":"option"},{"name":"option"},{"name":"option"},{"name":"option"}],"output":{"name":"user"}}],[11,"new","","",1,{"inputs":[{"name":"option"},{"name":"option"},{"name":"option"},{"name":"option"}],"output":{"name":"cluster"}}],[11,"default","","",1,{"inputs":[],"output":{"name":"cluster"}}],[11,"eq","","",3,{"inputs":[{"name":"config"},{"name":"config"}],"output":{"name":"bool"}}],[11,"ne","","",3,{"inputs":[{"name":"config"},{"name":"config"}],"output":{"name":"bool"}}],[11,"fmt","","",3,{"inputs":[{"name":"config"},{"name":"formatter"}],"output":{"name":"result"}}],[11,"from_std_path","","Reads a Config object from the default location on disk",3,{"inputs":[],"output":{"name":"result"}}],[11,"from_path","","Reads a Config object from a custom location on disk",3,{"inputs":[{"name":"p"}],"output":{"name":"result"}}],[11,"from_str","","Reads a Config object from a raw string payload",3,{"inputs":[{"name":"str"}],"output":{"name":"result"}}]],"paths":[[3,"Context"],[3,"Cluster"],[3,"User"],[3,"Config"],[4,"Error"],[4,"Content"]]};
initSearch(searchIndex);
