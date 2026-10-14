# PhotoCraft Android (prova de conceito)

App minimo em Rust (winit + wgpu/Vulkan). Abre uma tela e muda a cor ao toque.
Serve para validar a cadeia de build Android antes de integrar o nucleo do PhotoCraft.

## Gerar o APK pelo GitHub
1. Crie um repositorio com estes arquivos.
2. Aba **Actions > Build Android APK > Run workflow** (APK fica em "Artifacts").
3. Para uma Release: `git tag v0.0.1 && git push --tags`.

## Instalar
Baixe o .apk no celular, permita "instalar apps desconhecidos" e abra.

## Proximos passos
- Adicionar o PhotoCraft como dependencia (`git = "https://github.com/storytold/photocraft"`)
  e abrir/renderizar um PSD com os crates do nucleo.
- Selecionar arquivos via Storage Access Framework.
- Interface de toque.
