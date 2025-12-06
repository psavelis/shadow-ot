{{- define "shadow-server.name" -}}
shadow-server
{{- end -}}

{{- define "shadow-server.fullname" -}}
{{- printf "%s" (include "shadow-server.name" .) -}}
{{- end -}}
