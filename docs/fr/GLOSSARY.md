# Glossaire

- **Agrégat** : limite cohérente contenant entités et objets valeur.
- **Projection** : modèle de lecture dérivé des événements.
- **Événement** : enregistrement immuable décrivant quelque chose qui s'est produit.
- **Relecture** : processus d'application des événements passés pour reconstruire l'état.
- **Upcaster** : adaptateur mettant à niveau des événements d'anciens schémas.
- **Magasin d'événements** : persistance des événements de domaine ordonnés.
- **Commande** : requête de modification d'état du système.
- **Requête** : requête de lecture de l'état du système.
- **Idempotence** : propriété produisant le même résultat lorsqu'exécuté plusieurs fois.
- **ULID/UUID** : identifiants uniques utilisés pour entités et événements.
- **Politique de rétention** : règles de conservation ou suppression des événements.
- **Compaction** : nettoyage physique du stockage pour récupérer de l'espace.
- **Archivage** : déplacement des données de la couche chaude ou tiède vers la couche froide.
- **Politique de curiosité** : configure les comportements d'exploration de l'agent.
- **Politique de sécurité** : protège contre les actions nuisibles.
- **Curiosité** : exploration autonome du système à la recherche de nouvelles informations.
- **Stratégie d'exploration** : technique utilisée pour collecter des données (web crawl, requête API).
- **Plan d'exploration** : tâche planifiée définissant stratégie et cible d'exploration.
