# Model report for file:///tmp/top-repos-quality-repos-koe6a6vy/gatsby-ghost-blog.git HEAD f8d82237cb3277d95fa0a2093fa418bea4903ad6

### Dump

```json
{'created_at': '2021-09-02 00:41:08',
 'datasets': [],
 'dependencies': [],
 'description': 'Model bound to style.format.analyzer.FormatAnalyzer Lookout analyzer.',
 'environment': {'packages': 'ConfigArgParse==0.13.0 Jinja2==2.10 MarkupSafe==1.1.1 PyStemmer==1.3.0 PyYAML==5.1 Pympler==0.5 SQLAlchemy==1.2.10 SQLAlchemy-Utils==0.33.3 asdf==2.3.2 bblfsh==2.12.7 boto==2.49.0 boto3==1.9.130 botocore==1.12.130 cachetools==2.0.1 certifi==2019.3.9 chardet==3.0.4 clint==0.5.1 docker==3.7.0 docker-pycreds==0.4.0 dulwich==0.19.11 grpcio==1.19.0 grpcio-tools==1.19.0 humanfriendly==4.16.1 humanize==0.5.1 idna==2.8 jmespath==0.9.4 jsonschema==2.6.0 lookout-sdk==0.4.1 lookout-sdk-ml==0.19.0 lookout-style==0.2.0 lz4==2.1.6 modelforge==0.12.1 numpy==1.16.2 packaging==19.0 pandas==0.22.0 pip==19.0.3 protobuf==3.7.0 psycopg2-binary==2.7.5 pygtrie==2.3 pyparsing==2.3.1 python-dateutil==2.8.0 python-igraph==0.7.1.post6 pytz==2019.1 requests==2.21.0 requirements-parser==0.2.0 scikit-learn==0.20.1 scikit-optimize==0.5.2 scipy==1.2.1 semantic-version==2.6.0 setuptools==40.8.0 six==1.12.0 smart-open==1.8.1 sourced-ml==0.8.2 spdx==2.5.0 stringcase==1.2.0 tabulate==0.8.2 tqdm==4.31.1 '
                             'urllib3==1.24.1 websocket-client==0.55.0 xxhash==1.3.0',
                 'platform': 'Linux-4.15.0-135-generic-x86_64-with-Ubuntu-18.04-bionic',
                 'python': '3.6.7 (default, Oct 22 2018, 11:32:17) [GCC 8.2.0]'},
 'license': 'ODbL-1.0',
 'metrics': {},
 'model': 'style.format.analyzer.FormatAnalyzer',
 'references': [],
 'series': 'Lookout',
 'size': '16.8 kB',
 'tags': [],
 'uuid': 'd007d689-e2fb-428b-bc61-bd1117f2bf88',
 'vendor': 'source{d}',
 'version': [1]}
style.format.analyzer.FormatAnalyzer/[1] file:///tmp/top-repos-quality-repos-koe6a6vy/gatsby-ghost-blog.git f8d82237cb3277d95fa0a2093fa418bea4903ad6

# javascript
7 rules, avg.len. 6.7
## train
PPCR: 0.671252
### report
macro
{'f1-score': 0.5906700274964376,
 'precision': 0.5978911305019388,
 'recall': 0.5850927455153888,
 'support': 6685}
micro
{'f1-score': 0.9519820493642485,
 'precision': 0.9519820493642484,
 'recall': 0.9519820493642484,
 'support': 6685}
weighted
{'f1-score': 0.9386196132241902,
 'precision': 0.9272909114569577,
 'recall': 0.9519820493642484,
 'support': 6685}
### report_full
macro
{'f1-score': 0.3850874084724752,
 'precision': 0.5978911305019388,
 'recall': 0.30183284856126225,
 'support': 9959}
micro
{'f1-score': 0.7647200192261475,
 'precision': 0.9519820493642484,
 'recall': 0.6390199819258962,
 'support': 9959}
weighted
{'f1-score': 0.711608228654976,
 'precision': 0.8926962826272146,
 'recall': 0.6390199819258962,
 'support': 9959}
## test
PPCR: 0.664908
### report
macro
{'f1-score': 0.5272919976178593,
 'precision': 0.5446520929464705,
 'recall': 0.5497282608695653,
 'support': 1764}
micro
{'f1-score': 0.9399092970521541,
 'precision': 0.9399092970521542,
 'recall': 0.9399092970521542,
 'support': 1764}
weighted
{'f1-score': 0.9310697710334299,
 'precision': 0.9329561757178707,
 'recall': 0.9399092970521542,
 'support': 1764}
### report_full
macro
{'f1-score': 0.3681731696167974,
 'precision': 0.5446520929464705,
 'recall': 0.3063941833385068,
 'support': 2653}
micro
{'f1-score': 0.750735793525017,
 'precision': 0.9399092970521542,
 'recall': 0.6249528835280814,
 'support': 2653}
weighted
{'f1-score': 0.7015991779881496,
 'precision': 0.900162177767526,
 'recall': 0.6249528835280814,
 'support': 2653}
```

## javascript
### Summary
5 rules, avg.len. 5.8

| | |
|-|-|
|Min support|102|
|Max support|4381|
|Min confidence|0.9264705777168274|
|Max confidence|0.9940239191055298|

### Configuration

```json
{'feature_extractor': {'cutoff_label_support': 80,
                       'debug_parsing': False,
                       'label_composites': '<cut>',
                       'left_features': ['length',
                                         'diff_offset',
                                         'diff_col',
                                         'diff_line',
                                         'internal_type',
                                         'label',
                                         'reserved',
                                         'roles'],
                       'left_siblings_window': 5,
                       'no_labels_on_right': True,
                       'node_features': ['start_line', 'start_col'],
                       'parent_features': ['internal_type', 'roles'],
                       'parents_depth': 2,
                       'return_sibling_indices': False,
                       'right_features': ['length', 'internal_type', 'reserved', 'roles'],
                       'right_siblings_window': 5,
                       'select_features_number': 500,
                       'selected_features': '<cut>'},
 'line_length_limit': 500,
 'lines_ratio_train_trigger': 0.2,
 'lower_bound_instances': 500,
 'optimizer': {'base_model_name_categories': ['sklearn.ensemble.RandomForestClassifier',
                                              'sklearn.tree.DecisionTreeClassifier'],
               'cv': 3,
               'max_depth_categories': [None, 5, 10],
               'max_features_categories': [None, 'auto'],
               'min_samples_leaf_max': 120,
               'min_samples_leaf_min': 90,
               'min_samples_split_max': 240,
               'min_samples_split_min': 180,
               'n_iter': 50,
               'n_jobs': -1},
 'overall_size_limit': 5242880,
 'random_state': 42,
 'test_dataset_ratio': 0.2,
 'trainable_rules': {'attribute_similarity_threshold': 0.98,
                     'base_model_name': 'sklearn.tree.DecisionTreeClassifier',
                     'confidence_threshold': 0.8,
                     'min_samples_leaf': 90,
                     'min_samples_split': 180,
                     'n_estimators': 10,
                     'prune_attributes': True,
                     'prune_branches_algorithms': ['reduced-error'],
                     'prune_dataset_ratio': 0.2,
                     'top_down_greedy_budget': [False, 0.5]}}
```

### Rules

| rule number | description |
|----:|:-----|
| 1 | `  -1.reserved = :<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = ␣<br>Confidence: 0.994. Support: 251.` |
| 2 | `  -1.internal_type = StringLiteral<br>	∧ -1.reserved not in {:}<br>	∧ -3.roles in {IDENTIFIER}<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = "<br>Confidence: 0.980. Support: 174.` |
| 3 | `  -1.internal_type = StringLiteral<br>	∧ -1.reserved not in {:}<br>	∧ -3.roles not in {IDENTIFIER}<br>	∧ ^1.roles not in {DECLARATION}<br>⇒ y = '<br>Confidence: 0.926. Support: 102.` |
| 4 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved = ,<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +1.roles in {KEY}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.roles not in {DECLARATION, OPERATOR}<br>⇒ y = ⏎<br>Confidence: 0.974. Support: 134.` |
| 5 | `  -1.internal_type not in {StringLiteral}<br>	∧ -1.reserved not in {,, :, ;}<br>	∧ +1.internal_type not in {StringLiteral}<br>	∧ +2.reserved not in {=}<br>	∧ ^1.internal_type not in {ConditionalExpression, File, Program}<br>	∧ ^1.roles not in {DECLARATION, OPERATOR, STATEMENT}<br>⇒ y = ∅<br>Confidence: 0.953. Support: 4381.` |

### Note
All statistics are calculated with respect to the "analyze" config. This means that the rules were filtered by
`confidence_threshold` and `support_threshold` values.

<details>
    <summary>Machine-readable report</summary>
```json
{"javascript": {"avg_rule_len": 5.8, "max_conf": 0.9940239191055298, "max_support": 4381, "min_conf": 0.9264705777168274, "min_support": 102, "num_rules": 5}}
```
</details>
