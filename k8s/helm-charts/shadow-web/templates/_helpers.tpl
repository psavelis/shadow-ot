{{- define "shadow-web.name" -}}
shadow-web
{{- end -}}

{{- define "shadow-web.fullname" -}}
{{- printf "%s" (include "shadow-web.name" .) -}}
{{- end -}}
