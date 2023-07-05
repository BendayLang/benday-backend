# Questions

- Quelle peut etre une bonne facon de dire dans quel slot un node va ? (par example dans une variable, il y a 2 slots, le nom et la valeur, dans une fonction il y a le nom et les arguments, etc)

# TODO:

- [ ] begin the runner

# Informations

- On va separer de maniere tres distincte les parties suivantes:
  - Le runner (qui va executer le code)
  - Les models (qui feront office d'interface entre le runner et l'UI)
  - Le backend (un serveur qui fait le lien entre l'UI et le runner)
  - L'UI (qui va afficher les donnees et les modifier) (est-ce que l'app en generale est considere comme l'UI ?, gestion des fichiers, etc)
- L'UI va etre implementer en Rust, donc la 1er version, on aura pas besoin de backend, on peut appeler directement le runner depuis l'UI, mais dans le futur, on va vouloir avoir un backend pour pouvoir faire des requetes depuis n'importe quel langage et donc n'importe quel UI.
- L'UI sauvegarde ses donnees de positionnement dans un fichier a part.
